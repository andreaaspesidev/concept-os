# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

#
#   -------------------
#       AioSerial
#   -------------------
#   Based on https://pypi.org/project/aioserial/
import threading
import time

import serial
import asyncio as aio
from loguru import logger
from libraries.utils.closeable_queue import ClosableQueue, ClosableQueueClosed

DEBUG_STATUS = 0  # Debugs messages on status change
DEBUG_DATA_EXCHANGE = 1  # Debugs data flow


# aio serial port exception
class AIOSerialException(Exception):
    pass


# unable to open the port
class AIOSerialNotOpenException(AIOSerialException):
    pass


# port is already closed, no communication will take place
class AIOSerialClosedException(AIOSerialException):
    pass


# port fatal error
class AIOSerialErrorException(AIOSerialException):
    pass


# timeout exception
class AIOSerialTimeoutException(AIOSerialException):
    pass


# serial port asyncio implementation
def log_info(message):
    if DEBUG_STATUS:
        logger.info(message)


class AIOSerial:
    # create the serial port
    def __init__(self,
                 port=None,
                 baudrate=9600,
                 bytesize=serial.EIGHTBITS,
                 parity=serial.PARITY_NONE,
                 stopbits=serial.STOPBITS_ONE,
                 disable_dtr_rts: bool = False,
                 line_mode: bool = False):
        """
        Init a new serial port with asyncio support
        :param baudrate: Serial port's baudrate
        :param bytesize: Serial port's data bits
        :param parity: Serial port's parity
        :param stopbits: Serial port's stop bits
        :param disable_dtr_rts: If True, DTR and RTS will be cleared when port is opened
        :param line_mode: If True, \r are skipped and data is returned only after \n
        """
        try:
            self.port = port
            config = {'do_not_open': True,
                      'baudrate': baudrate,
                      'bytesize': bytesize,
                      'parity': parity,
                      'stopbits': stopbits}
            self.sp = serial.serial_for_url(self.port, **config)
        # re-raise the exception as AioSerialException
        except serial.SerialException as e:
            # log message
            logger.error(f"Error during init of '{self.port}'")
            # re-raise exception
            raise AIOSerialNotOpenException(f"Unable to init '{self.port}'") from e

        # are we working with the line mode? This will cause the read
        # functionality to return full text lines which is often desired
        # behavior for text protocols
        self.disable_dtr_rts = disable_dtr_rts
        self.line_mode = line_mode
        self.is_closing: threading.Event = threading.Event()
        self.is_closed = False

        # reception/transmission queue
        self._rxq = ClosableQueue()
        self._txq = ClosableQueue()

        self._rx_thread_fut = None
        self._tx_thread_fut = None

        # get current event loop
        self.loop = aio.get_running_loop()

    async def open(self):
        await self.loop.run_in_executor(None, self._open_port)
        # create receive and transmission tasks
        self._rx_thread_fut = self.loop.run_in_executor(None, self._rx_thread)
        self._tx_thread_fut = self.loop.run_in_executor(None, self._tx_thread)

        # log information
        log_info(f"Port '{self.port}' is now opened")

    def _open_port(self):
        # TODO: check if this call in an executor is thread safe (should be)
        # this may fail due to port not being present
        try:
            # open the serial port connection
            '''
                Bugfix of PySerial
                ==================
                On Windows, PySerial opens serial port with DTR and RTS set, and for
                this reason Arduino resets when the port is opened. Also, it's not possible
                to communicate due to RTS.
                We create the serial port using serial_for_url to enhance support for special ports
            '''
            if self.disable_dtr_rts:
                self.sp.setDTR(False)
                self.sp.setRTS(False)
            self.sp.open()
            # port was not opened
            if not self.sp.is_open:
                raise AIOSerialException()
        # re-raise the exception as AioSerialException
        except (AIOSerialException, serial.SerialException) as e:
            # log message
            logger.error(f"Error during opening '{self.port}'")
            # re-raise exception
            raise AIOSerialNotOpenException(f"Unable to open port '{self.port}'") from e

    # called when entering 'async with' block
    async def __aenter__(self):
        # all was done in the constructor, so we can simply return the opened port
        return self

    # called when exiting 'async with' block
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        # close the port
        await self.close()

    # close the serial port, do the cleanup
    async def close(self):
        # if already closed or closing, just return
        if self.is_closed or self.is_closing.is_set():
            return
        # set flag used to close read routine
        self.is_closing.set()
        # port is open?
        if self.sp.is_open:
            # cancel ongoing read-write operation to ensure that rx thread is
            # not stuck inside the read() function
            self.sp.cancel_read()

        # close both queues
        self._txq.close()
        self._rxq.close()

        # wait for the rx/tx thread to end, these need to be gathered to
        # collect all the exceptions
        if self._tx_thread_fut is not None and self._rx_thread_fut is not None:
            await aio.gather(self._tx_thread_fut, self._rx_thread_fut)

        # ensure that we call the close function no matter what
        self.sp.close()

        # mark as closed
        self.is_closing.clear()
        self.is_closed = True

        # log information
        log_info(f"Port '{self.port}' is now closed")

    # close port filtering exceptions
    async def force_close(self):
        try:
            await self.close()
        except OSError:
            pass

    # reception thread
    def _rx_thread(self):
        # putting into the rx queue may fail, read may fail as well
        try:
            last_data = b""
            while not self.is_closing.is_set():
                if self.sp.inWaiting() > 0:
                    # read from the port
                    char = self.sp.read()
                    # Read function stops if data is available or is port is closing
                    # as consequence of sp.cancel_read()
                    if self.is_closing.is_set():
                        break

                    if self.line_mode:
                        # if line mode, terminators supported are \n \r\n
                        if char == b'\n':  # Got data on \n
                            self.add_data(last_data)
                            last_data = b""
                        elif char == b'\r':  # Ignore \r
                            pass
                        else:
                            last_data += char
                    else:
                        self.add_data(char)
                else:
                    time.sleep(0.01)

        # queue closed exception raised? exit the loop gracefully (no
        # exceptions) as this can only happen when the port is getting
        # intentionally closed
        except ClosableQueueClosed:
            pass
        # serial port exceptions, all of these notify that we are in some
        # serious trouble
        except serial.SerialException:
            # log message
            logger.error(f"RX error on port '{self.port}'")
            # create the exception of our own
            # e = AIOSerialErrorException("Serial Port Reception Error")
            # close the queue
            # optional close only queue aio.run_coroutine_threadsafe(aio.coroutine(self._rxq.close)(e), self.loop)
            # must close invoke on the main loop, as this function runs in an executor
            aio.run_coroutine_threadsafe(self.close(), self.loop)

        # log information
        log_info(f"RX Thread has ended for port '{self.port}'")

    def add_data(self, data):
        # log information
        if DEBUG_DATA_EXCHANGE:
            logger.debug(f"{self.port} > {data}")

        # try putting the data to the queue, this will raise an
        # exception when queue is closed which is in turn caused by the
        # port itself getting closed. we use the result to
        # raise the exception if any was thrown by the _rxq.put()
        aio.run_coroutine_threadsafe(self._rxq.put(data),
                                     self.loop).result()

    # transmission thread
    def _tx_thread(self):
        # this may fail due to serial port or queue
        try:
            # this loop can only be broken by exceptions
            while not self.is_closing.is_set():
                # try getting data from the queue, this will raise an
                # exception when queue is closed due to the fact that port is
                # getting closed
                data = aio.run_coroutine_threadsafe(self._txq.get(),
                                                    self.loop).result()
                # write the data to the serial port
                self.sp.write(data)
                # log information
                if DEBUG_DATA_EXCHANGE:
                    logger.debug(f"{self.port} < {data}")
        # queue closed exception raised? exit the loop gracefully (no
        # exceptions) as this can only happen when the port is getting
        # intentionally closed
        except ClosableQueueClosed or aio.CancelledError:
            pass
        # serial port related exceptions
        except serial.SerialException:
            # log message
            logger.error(f"TX error on port '{self.port}'")
            # create the exception of our own
            # e = AIOSerialErrorException("Serial Port Transmission Error")
            # close the queue
            # aio.run_coroutine_threadsafe(aio.coroutine(self._txq.close)(e), self.loop)
            aio.run_coroutine_threadsafe(self.close(), self.loop)

        # log information
        log_info(f"TX Thread has ended for port '{self.port}'")

    # read from serial port
    async def read(self):
        # port might get closed
        try:
            # get data from the queue
            return await self._rxq.get()
        # closed queue means closed port
        except ClosableQueueClosed:
            raise AIOSerialClosedException("Serial Port is closed")

    # flush read buffer
    def read_flush(self):
        self._rxq.clear()

    # read from serial port for the maximum time
    async def read_for(self, timeout=None):
        try:
            # use asyncio function. If timeout=None equals to read
            return await aio.wait_for(self.read(), timeout=timeout)
        except aio.TimeoutError:
            raise AIOSerialTimeoutException("Read timeout triggered")

    # write to serial port
    async def write(self, data):
        # unsupported type of data
        if not isinstance(data, (bytes, bytearray)):
            raise TypeError("Data must be of type bytes or bytearray")
        # port might get closed
        try:
            # put data to port
            await self._txq.put(data)
        # closed queue means closed port
        except ClosableQueueClosed:
            raise AIOSerialClosedException("Serial Port is closed")

    # write line to serial port, with \n char
    async def println(self, data):
        # unsupported type of data
        if not isinstance(data, (bytes, bytearray)):
            raise TypeError("Data must be of type bytes or bytearray")
        # write adding new line char
        await self.write(data + b'\n')

    # checks if serial is opened
    def is_opened(self):
        # just return internal serial status
        return self.sp.is_open
