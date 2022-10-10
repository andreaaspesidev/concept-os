import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/constants.dart';
import 'package:bthermo_app/integrations/mqtt.dart';
import 'package:bthermo_app/redux/state/app_state.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_data.dart';

class RequestTemperatureAction extends ReduxAction<AppState> {
  RequestTemperatureAction();

  @override
  Future<AppState?> reduce() async {
    // Ask for temperature
    var clockMsg = Uint8Buffer();
    clockMsg.add(REQUEST_TEMP_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, clockMsg);
    await Future.delayed(const Duration(milliseconds: 100));
    return null;
  }
}
