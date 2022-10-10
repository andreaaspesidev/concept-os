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
    multiplex_enable: bool,
    mqtt_client: MQTTConnector,
    mqtt_out_queue: aio.Queue,
    serial_out_queue: aio.Queue,
    mqtt_root: str
):
    # Setup message handler
    async def _on_mqtt_data(topic: str, payload: bytes) -> None:
        # If simple case, then just redirect payload to serial output
        if not multiplex_enable:
            serial_out_queue.put_nowait({
                'data': payload
            })
            return

        # Read the component id from the topic
        match = re.match(f'{mqtt_root}/([^\/]+)\/in', topic)
        if match is None:
            logger.warning("Wrong topic structure")
            return
        try:
            component_id = int(match.group(1))
            # Add the payload to the output queue
            serial_out_queue.put_nowait({
                'id': component_id,
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
        # If simple case, just use a single topic
        topic = f"{mqtt_root}/out"
        if multiplex_enable:
            topic = f"{mqtt_root}/{pkt['id']}/out"
        # Launch publish
        if not mqtt_client.publish(
            topic=topic,
            payload=pkt['data'],
            qos=2,
            retain=False
        ):
            logger.warning("Cannot publish packet to MQTT")


async def serial_loop(
    multiplex_enable: bool,
    serial_port: AIOSerial,
    serial_out_queue: aio.Queue,
    mqtt_out_queue: aio.Queue
):
    # Decoder task
    async def _decoder() -> None:
        in_buffer = bytearray()
        last_component_id = None
        last_pkt_len = 0
        try:
            logger.info("Serial decoder online")
            while True:
                byte = await serial_port.read()
                in_buffer.extend(byte)

                # If no multiplex, then just read data length
                if not multiplex_enable:
                    if last_pkt_len == 0:
                        if len(in_buffer) < 2:
                            continue
                        last_pkt_len = int.from_bytes(
                            in_buffer[0:2], byteorder='big')    
                        # Skip this data
                        in_buffer = in_buffer[2:]
                    if len(in_buffer) == last_pkt_len:
                        mqtt_out_queue.put_nowait({
                            'data': in_buffer[:]
                        })
                        last_pkt_len = 0
                        in_buffer = bytearray()
                    continue

                # Check if we are already reading a packet
                if last_component_id is None:
                    # Wait until we have a full header
                    # (2 * 16bits: 4 bytes)
                    if len(in_buffer) < 4:
                        continue
                    # Read the header
                    last_component_id = int.from_bytes(
                        in_buffer[0:2], byteorder='big')
                    last_pkt_len = int.from_bytes(
                        in_buffer[2:4], byteorder='big')
                    # Skip this data
                    in_buffer = in_buffer[4:]
                # Decode a packet
                if len(in_buffer) == last_pkt_len:
                    mqtt_out_queue.put_nowait({
                        'id': last_component_id,
                        'data': in_buffer[:]
                    })
                    in_buffer = bytearray()
                    last_component_id = None

        except AIOSerialException:
            logger.error("Error in serial port reading!")
            sys.exit(-2)

    # Encoder task
    async def _encoder() -> None:
        logger.info("Serial encoder online")
        while True:
            pkt = await serial_out_queue.get()
            # Simple case, no mux
            if not multiplex_enable:
                await serial_port.write(pkt['data'])
                continue

            # Write header first
            c_id = int(pkt['id']).to_bytes(2, byteorder='big')
            c_len = int(len(pkt['data'])).to_bytes(2, byteorder='big')
            await serial_port.write(c_id)
            await serial_port.write(c_len)
            # Then write data
            await serial_port.write(pkt['data'])
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

    # Read mode from settings
    multiplex_enable = settings.get('multiplex/enable', False)

    # Now that everything is online, perform the subscriptions
    if not multiplex_enable:
        # Subscribe also to the single one
        if not await mqtt_client.subscribe(mqtt_root + "/in", qos=2):
            logger.error("Cannot subscribe to MQTT")
            sys.exit(-1)
    # If multiplex is not enabled, all will be mapped to /in
    if not await mqtt_client.subscribe(mqtt_root + "/+/in", qos=2):
        logger.error("Cannot subscribe to MQTT")
        sys.exit(-1)

    # Create queues
    serial_out_queue = aio.Queue(1000)
    mqtt_out_queue = aio.Queue(1000)

    # Begin checking MQTT and Serial traffic
    aio.create_task(mqtt_loop(
        multiplex_enable=multiplex_enable, 
        mqtt_client=mqtt_client,
        mqtt_out_queue=mqtt_out_queue,
        serial_out_queue=serial_out_queue,
        mqtt_root=mqtt_root))

    aio.create_task(serial_loop(
        multiplex_enable=multiplex_enable, 
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
