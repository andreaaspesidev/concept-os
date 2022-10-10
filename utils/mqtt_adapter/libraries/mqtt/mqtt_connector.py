import asyncio as aio
from typing import Coroutine, Any, Callable, Optional, Pattern, Dict

from libraries.mqtt.gmqtt import gmqtt
from libraries.mqtt.gmqtt.gmqtt.client import Client as MQTTClient
from libraries.mqtt.gmqtt.gmqtt.mqtt.constants import PubRecReasonCode, MQTTv311

from libraries.utils.aio_utils import wait_cancel_others
from libraries.utils.mqtt_utils import topic_to_pattern


class MQTTConnector:
    """
    Asyncio MQTT library.
    This class is a wrapper for an asyncio MQTT library, providing standard functionalities
    for correct module execution. In this way the underneath library can be replaced and no
    changes will be done in the module's code.
    """
    def __init__(self,
                 client_id: str,
                 will_topic: str = None,
                 will_offline_payload: str = None,
                 clean_session: bool = True,
                 resubscribe_after_reconnection: bool = True,
                 default_timeout: float = 10):
        # Init vars
        self._on_message_handler = None
        self._on_connected_handler = None
        self._on_disconnected_handler = None
        self._will_topic = will_topic
        self._online_payload = None
        self._default_timeout = default_timeout
        self._resubscribe_after_reconnection = resubscribe_after_reconnection
        # Init client
        will_message = None
        if will_topic is not None and will_offline_payload is not None:
            will_message = gmqtt.Message(
                topic=will_topic,
                qos=2,
                retain=True,
                payload=will_offline_payload
            )
        self._client_id = client_id
        self._will_message = will_message
        self._clean_session = clean_session
        # Set properties
        self._mqtt_client: Optional[MQTTClient] = None
        self._subscribe_mids = dict()       # mid -> wait event
        self._unsubscribe_mids = dict()     # mid -> wait event
        self._connection_lost_event = aio.Event()
        self._new_subscribe_mid = aio.Event()
        self._new_unsubscribe_mid = aio.Event()
        self._subscribed_topics: Dict[str, Pattern] = dict()     # list used to restore subscriptions after reconnect

    # -------------------
    # MQTT callbacks
    # -------------------
    async def _on_mqtt_connect(self):
        # Restore subscriptions
        if self._resubscribe_after_reconnection:
            for topic in self._subscribed_topics:
                await self.subscribe(topic=topic)
        # Report status online
        if self._online_payload is not None and self._will_topic is not None:
            self.publish(
                topic=self._will_topic,
                payload=self._online_payload,
                qos=2,
                retain=True
            )
        if self._on_connected_handler is not None:
            self._on_connected_handler()

    def _on_connected(self):
        self._connection_lost_event.clear()
        self._subscribe_mids = dict()
        self._unsubscribe_mids = dict()
        aio.create_task(self._on_mqtt_connect())

    def _on_disconnected(self):
        self._connection_lost_event.set()
        if self._on_disconnected_handler is not None:
            self._on_disconnected_handler()

    async def _on_message(self, client, topic, payload, qos, properties):
        if self._on_message_handler is not None:
            # Task creation could lead to unordered scheduling in some Python implementation
            # in that case, modify the gmqtt library and all the on_message chain of this library
            # and append {topic, payload} to a queue from the on_message task, then process the queue
            # using a separate task. In this way, the on_message task is not stopped, and new messages
            # can be accepted during the handling of this message, but order is preserved as the queue
            # if FIFO
            await self._on_message_handler(topic, payload)
        # Use not blocking code in _on_message_handler to avoid hanging on the broker
        # NOT NEEDED is gmqtt is set in optimistic acknowledgement
        return PubRecReasonCode.SUCCESS

    def _on_subscribe(self, client, mid, qos, properties):
        # Set event corresponding to this mid
        if mid not in self._subscribe_mids:
            # If not present, wake up waiters
            self._subscribe_mids[mid] = aio.Event()
            self._new_subscribe_mid.set()
            self._new_subscribe_mid.clear()
        self._subscribe_mids[mid].set()

    def _on_unsubscribe(self, client, mid, qos):
        # Set event corresponding to this mid
        if mid not in self._unsubscribe_mids:
            # If not present, wake up waiters
            self._unsubscribe_mids[mid] = aio.Event()
            self._new_unsubscribe_mid.set()
            self._new_unsubscribe_mid.clear()
        self._unsubscribe_mids[mid].set()

    # -------------------
    # Callbacks setters
    # -------------------
    def on_connect_handler(self, on_connected_handler: Callable[[], None]):
        self._on_connected_handler = on_connected_handler

    def on_disconnect_handler(self, on_disconnected_handler: Callable[[], None]):
        self._on_disconnected_handler = on_disconnected_handler

    def on_message_handler(self, on_message_handler: Callable[[str, bytes], Coroutine[Any, Any, None]]):
        self._on_message_handler = on_message_handler

    # -------------------
    # Public methods
    # -------------------
    async def connect(self,
                      server_ip: str,
                      server_port: int,
                      username: str,
                      password: str,
                      online_payload: str = None) -> bool:
        # First close
        await self.close()
        # Create a new client
        self._mqtt_client = MQTTClient(
            client_id=self._client_id,  # Using device ID as MQTT ID
            will_message=self._will_message,
            clean_session=self._clean_session
        )
        # Link callbacks
        self._mqtt_client.on_message = self._on_message
        self._mqtt_client.on_subscribe = self._on_subscribe
        self._mqtt_client.on_unsubscribe = self._on_unsubscribe
        self._mqtt_client.on_connect = lambda *args, **kwargs: self._on_connected()
        self._mqtt_client.on_disconnect = lambda *args, **kwargs: self._on_disconnected()
        # Set credentials
        self._mqtt_client.set_auth_credentials(
            username=username,
            password=password
        )
        # Connect
        try:
            self._online_payload = online_payload
            # Online payload will be set automatically
            await self._mqtt_client.connect(
                host=server_ip,
                port=server_port,
                version=MQTTv311,
                raise_exc=True
            )
            return self._mqtt_client.is_connected
        except OSError:
            # Disconnect for a bug in gmqtt library. Resend task is created before a successful connection is made
            await self._mqtt_client.disconnect(disconnect_with_will=True)
            self._mqtt_client = None
            # In case of connection error, return false
            return False

    async def subscribe(self, topic: str, qos: int = 2) -> bool:
        # Start by checking if we are connected
        if not self.is_connected():
            return False
        # Send subscribe request
        mid = self._mqtt_client.subscribe(
            subscription_or_topic=topic,
            qos=qos
        )
        # Wait for response or disconnection
        result = await self._wait_for_sub_response(
            is_subscription=True,
            mid=mid,
            timeout=self._default_timeout
        )
        if result:
            self._subscribed_topics[topic] = topic_to_pattern(topic=topic)
        return result

    async def unsubscribe(self, topic: str) -> bool:
        # Start by checking if we are connected
        if self._mqtt_client is None:
            # If we are not connected, we can still remove the subscription
            # as if reconnect is enabled, equals not to resubscribe when we go back online
            if topic in self._subscribed_topics:
                self._subscribed_topics.pop(topic)
                return True
            return False
        elif not self._mqtt_client.is_connected:
            return False
        # Send unsubscribe request
        mid = self._mqtt_client.unsubscribe(
            topic=topic
        )
        # Wait for response or disconnection
        result = await self._wait_for_sub_response(
            is_subscription=False,
            mid=mid,
            timeout=self._default_timeout
        )
        if result:
            if topic in self._subscribed_topics:
                self._subscribed_topics.pop(topic)
        return result

    async def clear_subscriptions(self) -> bool:
        for topic in self._subscribed_topics:
            if not self.unsubscribe(topic=topic):
                return False
        return True

    def is_subscribed(self, topic: str) -> bool:
        """
        Checks if the client is already subscribed to the topic,
        checking also wildcards.
        :param topic: topic. No wildcards allowed
        :return: true if a match is found, false otherwise
        """
        for t in self._subscribed_topics:
            if self._subscribed_topics[t].match(string=topic):
                return True
        return False

    def publish(self, topic: str, payload: Any, qos: int = 2, retain: bool = False) -> bool:
        if not self.is_connected():
            return False
        # Actually there is a resend procedure, so messages may not be sent immediately
        self._mqtt_client.publish(
            message_or_topic=topic,
            qos=qos,
            payload=payload,
            retain=retain
        )
        return True

    def is_connected(self) -> bool:
        return self._mqtt_client is not None and self._mqtt_client.is_connected

    async def close(self, discard_subscriptions: bool = False):
        if self._mqtt_client is not None:
            await self._mqtt_client.disconnect(disconnect_with_will=True)    # Exit with will
        # Clear data
        self._subscribe_mids = dict()
        self._unsubscribe_mids = dict()
        self._connection_lost_event.clear()
        if discard_subscriptions:
            self._subscribed_topics.clear()
        self._mqtt_client = None

    # ----------------
    # Utils
    # ----------------
    async def _wait_for_mid(self, is_subscription: bool, mid: int):
        on_dict = self._subscribe_mids if is_subscription else self._unsubscribe_mids
        on_new_element = self._new_subscribe_mid if is_subscription else self._new_unsubscribe_mid
        # Wait for element if not created
        while mid not in on_dict:
            await on_new_element.wait()
        await on_dict[mid].wait()

    async def _wait_for_mid_decorated(self, is_subscription: bool, mid: int, timeout: float = None):
        try:
            await aio.wait_for(self._wait_for_mid(is_subscription, mid), timeout=timeout)
        except aio.TimeoutError:
            pass

    async def _wait_for_sub_response(self, is_subscription: bool, mid: int, timeout: float = None):
        task_e = aio.create_task(self._wait_for_mid_decorated(is_subscription, mid, timeout))
        task_c = aio.create_task(self._connection_lost_event.wait())
        await wait_cancel_others([task_e, task_c], return_when=aio.FIRST_COMPLETED)
        # Clear and return result
        on_dict = self._subscribe_mids if is_subscription else self._unsubscribe_mids
        if mid not in on_dict:
            return False
        else:
            on_dict.pop(mid)
        return task_c.cancelled()

