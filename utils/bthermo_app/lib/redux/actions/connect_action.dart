import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/constants.dart';
import 'package:bthermo_app/integrations/mqtt.dart';
import 'package:bthermo_app/redux/events/on_disconnect_event.dart';
import 'package:bthermo_app/redux/events/on_message_event.dart';
import 'package:bthermo_app/redux/state/app_state.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:mqtt_client/mqtt_client.dart';

class ConnectAction extends ReduxAction<AppState> {
  ConnectAction();

  @override
  Future<AppState> reduce() async {
    // Setup MQTT client
    client.logging(on: true);
    client.setProtocolV311();
    client.keepAlivePeriod = 20;
    // Register handler for later event
    client.onDisconnected = () => {
          // Dispatch disconnect action
          dispatch(OnDisconnectEvent())
        };
    final connMess = MqttConnectMessage().startClean();
    client.connectionMessage = connMess;

    // Try connecting
    try {
      await client.connect();
    } on NoConnectionException catch (e) {
      // Raised by the client when connection fails.
      print('EXAMPLE::client exception - $e');
      client.disconnect();
    } on SocketException catch (e) {
      // Raised by the socket layer
      print('EXAMPLE::socket exception - $e');
      client.disconnect();
    }
    // Check result
    if (client.connectionStatus!.state == MqttConnectionState.connected) {}
    // Subscribe to topic
    client.subscribe(THERMO_OUT_TOPIC, MqttQos.exactlyOnce);
    // Register handler
    client.updates!.listen((List<MqttReceivedMessage<MqttMessage?>>? c) {
      c?.forEach((msg) {
        var payload = msg.payload as MqttPublishMessage;
        dispatch(OnMessageEvent(
            dataBytes: payload.payload.message.buffer.asUint8List()));
      });
    });
    return state.copy(connected: true);
  }

  @override
  void after() {
    // Activate status timer
    startStatusTimer();
    // Navigate to home page
    dispatch(NavigateAction.pushNamedAndRemoveAll("/device"));
  }
}
