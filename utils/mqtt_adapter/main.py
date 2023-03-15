# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

from array import array
import asyncio as aio
import argparse
import sys
import re

from loguru import logger
from src.utils.paths import configure_path
from src.utils.settings import Settings
from src.utils.logger import init_logger

from libraries.aio_serial.aio_serial import AIOSerial, AIOSerialNotOpenException, AIOSerialException
from libraries.mqtt.mqtt_connector import MQTTConnector


async def mqtt_loop(
    mqtt_client: MQTTConnector,
    mqtt_out_queue: aio.Queue,
    serial_out_queue: aio.Queue,
    mqtt_root: str
):
    # Setup message handler
    async def _on_mqtt_data(topic: str, payload: bytes) -> None:
        # Read the channel id from the topic
        match = re.match(f'{mqtt_root}/([^\/]+)\/in', topic)
        if match is None:
            logger.warning("Wrong topic structure")
            return
        try:
            channel_id = int(match.group(1))
            # Add the payload to the output queue
            serial_out_queue.put_nowait({
                'id': channel_id,
                'data': payload
            })
        except ValueError:
            logger.warning(f"Invalid topic selected: {topic}")
            return
    # Register the handler
    mqtt_client.on_message_handler(_on_mqtt_data)
    # Start the sender loop
    logger.info("MQTT online")
    while True:
        pkt = await mqtt_out_queue.get()
        topic = f"{mqtt_root}/{pkt['id']}/out"
        # Launch publish
        if not mqtt_client.publish(
            topic=topic,
            payload=pkt['data'],
            qos=2,
            retain=False
        ):
            logger.warning("Cannot publish packet to MQTT")


#   Packet format
#   +----------+------------+---------------+----------+--------+
#   | Preamble | Channel ID | Packet Length |   Data   | CRC-8  |
#   +----------+------------+---------------+----------+--------+
#   | 4 bytes  | 2 bytes    | 2 bytes       | pl_bytes | 1 byte |
#   +----------+------------+---------------+----------+--------+
#

# Optimized Dallas (now Maxim) iButton 8-bit CRC calculation.
# Polynomial: x^8 + x^5 + x^4 + 1 (0x8C)
# Initial value: 0x0
def crc8_update(crc, byte) -> int:
    tmp = crc ^ byte
    for _ in range(0,8):
        if tmp & 0x01 == 1:
            tmp = (tmp >> 1) ^ 0x8C
        else:
            tmp >>= 1
    return tmp

async def serial_loop(
    serial_port: AIOSerial,
    serial_out_queue: aio.Queue,
    mqtt_out_queue: aio.Queue
):
    # Decoder task
    async def _decoder() -> None:
        in_buffer = bytearray()
        last_channel_id = None
        last_pkt_len = 0
        preamble_cnt = 0
        crc8 = 0
        try:
            logger.info("Serial decoder online")
            while True:
                byte = await serial_port.read()
                in_buffer.extend(byte)
                # Check if we are already reading a packet
                if last_channel_id is None:
                    # Wait for a preamble, discarding the bytes
                    if preamble_cnt != 4:
                        # Check if this byte is a preamble byte
                        if in_buffer[0] == 0xAA:
                            in_buffer = in_buffer[1:]
                            preamble_cnt += 1
                            continue # Wait for next byte
                        else:
                            # Mistaken sth else for the preamble, restart
                            in_buffer = in_buffer[1:]
                            preamble_cnt = 0
                            continue
                    
                    # Wait until we have a full header
                    # (2 * 16bits: 4 bytes)
                    if len(in_buffer) < 4:
                        continue
                    # Read the header
                    last_channel_id = int.from_bytes(
                        in_buffer[0:2], byteorder='big')
                    last_pkt_len = int.from_bytes(
                        in_buffer[2:4], byteorder='big')
                    crc8 = 0
                    # Update crc with the header
                    for i in range(0,4):
                        crc8 = crc8_update(crc8,in_buffer[i])
                    # Skip this data
                    in_buffer = in_buffer[4:]
                
                # Decode a packet, first update crc
                if len(in_buffer) == last_pkt_len + 1:
                    # Check crc, otherwise discard everything and wait for a new preamble
                    if in_buffer[last_pkt_len] != crc8:
                        preamble_cnt = 0    # Wait for next preamble
                        last_channel_id = None 
                        logger.warning("Discarding package for wrong CRC")
                        continue
                    # Otherwise push data
                    mqtt_out_queue.put_nowait({
                        'id': last_channel_id,
                        'data': in_buffer[:-1]
                    })
                    # Empty everything
                    in_buffer = bytearray()
                    preamble_cnt = 0    # Wait for next preamble
                    last_channel_id = None
                elif len(in_buffer) > 0:
                    crc8 = crc8_update(crc8,in_buffer[len(in_buffer)-1])

        except AIOSerialException:
            logger.error("Error in serial port reading!")
            sys.exit(-2)

    # Encoder task
    async def _encoder() -> None:
        logger.info("Serial encoder online")
        while True:
            pkt = await serial_out_queue.get()
            # Write preamble + header
            c_preamble = b"\xAA"*4
            c_id = int(pkt['id']).to_bytes(2, byteorder='big')
            c_len = int(len(pkt['data'])).to_bytes(2, byteorder='big')
            # Compute CRC
            crc8 = 0
            for i in range(0,2):
                crc8 = crc8_update(crc8,c_id[i])
            for i in range(0,2):
                crc8 = crc8_update(crc8,c_len[i])
            for i in range(0,len(pkt['data'])):
                crc8 = crc8_update(crc8,pkt['data'][i])
            # Write preamble + header
            await serial_port.write(c_preamble)
            await serial_port.write(c_id)
            await serial_port.write(c_len)
            # Then write data
            await serial_port.write(pkt['data'])
            # Then write crc8
            await serial_port.write(crc8.to_bytes(1, byteorder='big'))
    # Start tasks
    aio.create_task(_encoder())
    aio.create_task(_decoder())
    logger.debug


async def init(settings: Settings):
    # Get serial port parameters
    port = settings.get('serial/port_name')
    if port is None:
        logger.error("Missing serial port in the configuration file")
        sys.exit(-1)
    # Open serial port
    serial = AIOSerial(port=port, baudrate=settings.get(
        'serial/baudrate', 9600), line_mode=False)
    try:
        await serial.open()
    except (AIOSerialNotOpenException):
        logger.error(f"Cannot open serial port '{port}'")
        sys.exit(-1)

    # Get MQTT parameters
    mqtt_ip = settings.get('mqtt/server_ip')
    if mqtt_ip is None:
        logger.error("Missing MQTT Server IP")
        sys.exit(-1)
    mqtt_port = settings.get('mqtt/server_port', default_value=1883)
    mqtt_user = settings.get('mqtt/username', default_value='mqtt')
    mqtt_password = settings.get('mqtt/password', default_value='mqtt')
    mqtt_root = settings.get('mqtt/root_topic')
    if mqtt_root is None:
        logger.error("Missing MQTT Root Topic")
        sys.exit(-1)

    # Try to connect to MQTT
    mqtt_client = MQTTConnector(
        client_id="concept-os-adapter",
        will_topic=mqtt_root + "/available",
        will_offline_payload='0'
    )
    if not await mqtt_client.connect(
        server_ip=mqtt_ip,
        server_port=mqtt_port,
        username=mqtt_user,
        password=mqtt_password,
        online_payload='1'
    ):
        logger.error("Cannot connect to MQTT")
        sys.exit(-1)

    # Now that everything is online, perform the subscriptions
    if not await mqtt_client.subscribe(mqtt_root + "/+/in", qos=2):
        logger.error("Cannot subscribe to MQTT")
        sys.exit(-1)

    # Create queues
    serial_out_queue = aio.Queue(1000)
    mqtt_out_queue = aio.Queue(1000)

    # Begin checking MQTT and Serial traffic
    aio.create_task(mqtt_loop(
        mqtt_client=mqtt_client,
        mqtt_out_queue=mqtt_out_queue,
        serial_out_queue=serial_out_queue,
        mqtt_root=mqtt_root))

    aio.create_task(serial_loop(
        serial_port=serial,
        serial_out_queue=serial_out_queue,
        mqtt_out_queue=mqtt_out_queue))

    logger.info("Loading completed")


def start():
    # Configure path
    configure_path()

    # Parse command line args
    parser = argparse.ArgumentParser(
        description='Concept-OS Serial to MQTT Adapter')
    parser.add_argument('-c', '--config',
                        nargs="?",
                        type=str,
                        required=False,
                        default="settings.yaml",
                        help='Specify a custom config path')
    args = vars(parser.parse_args())

    # Load settings
    settings = Settings()
    if not settings.load(args["config"]):
        logger.warning(f"Cannot load settings file '{args['config']}'")

    # Init logger
    init_logger(settings)

    # Get asyncio loop and link start function
    loop = aio.get_event_loop()
    loop.create_task(init(settings=settings))

    try:
        # Run asyncio run forever
        loop.run_forever()
    except KeyboardInterrupt:
        sys.exit(-1)
    finally:
        loop.close()


if __name__ == '__main__':
    start()
