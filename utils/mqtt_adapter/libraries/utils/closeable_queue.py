from libraries.utils.aio_utils import *


# queue closed exception
class ClosableQueueClosed(Exception):
    pass


# queue with option for closing. perfect for representing data channels. use
# close method to mark the channel as closed. After 'closing' putting data will 
# raise an ClosableQueueClosed exception, BUT you can still get data from the 
# queue as long as it's there. The next call to 'get' after the queue has been 
# emptied will raise ClosableQueueClosed exception.
class ClosableQueue(aio.Queue):
    # constructor
    def __init__(self, *args, **kwargs):
        # this event will be used to notify about channel that is represented by
        # the queue being closed
        self._closed_ev = aio.Event()
        # queue exception, can be set by using the 'close' method. after setting
        # every attempt to put more data to the queue will result in re-raising
        # the exception
        self._exc = None
        # create queue
        super().__init__(*args, **kwargs)

    # close the queue channel
    def close(self, exc=None):
        # .. by emitting the closed event
        self._closed_ev.set()
        # store the exception as well. if none is provided then use the default
        # exception for the queue closure
        self._exc = exc or ClosableQueueClosed()

    # get value without waiting
    def get_nowait(self):
        # queue empty and closed?
        if super().qsize() == 0 and self._closed_ev.is_set():
            raise self._exc
        # use the underlying implementation
        return super().get_nowait()

    # clears the queue
    # TODO: check if this function can cause problems
    def clear(self):
        # for each element contained
        for _ in range(super().qsize()):
            # pop the element
            super().get_nowait()

    # get value from queue, values can be fetched even when the queue is closed
    async def get(self):
        # got elements in the queue, then there is no need to wait.
        if super().qsize() > 0:
            return super().get_nowait()
        # queue closed
        elif self._closed_ev.is_set():
            raise self._exc

        # create two tasks
        task_q = aio.create_task(super().get())
        task_e = aio.create_task(self._closed_ev.wait())
        # join all tasks
        await wait_cancel_others([task_q, task_e],
                                 return_when=aio.FIRST_COMPLETED)
        # got the closing event risen?
        if not task_q.cancelled():
            return task_q.result()
        # return the result
        else:
            raise self._exc

    # put data into queue without waiting
    def put_nowait(self, item):
        # queue is closed
        if self._closed_ev.is_set():
            raise self._exc
        # use underlying implementation
        super().put_nowait(item)

    # put value into queue
    async def put(self, item):
        # queue closed, no point in further processing
        if self._closed_ev.is_set():
            raise self._exc

        # # create two tasks
        task_q = aio.create_task(super().put(item))
        task_e = aio.create_task(self._closed_ev.wait())
        # join all tasks
        await wait_cancel_others([task_q, task_e],
                                 return_when=aio.FIRST_COMPLETED)
        # got the closing event risen?
        if not task_q.cancelled():
            return task_q.result()
        # return the result
        else:
            raise self._exc
