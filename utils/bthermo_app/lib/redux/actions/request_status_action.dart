import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/constants.dart';
import 'package:bthermo_app/integrations/mqtt.dart';
import 'package:bthermo_app/redux/state/app_state.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_data.dart';

class RequestStatusAction extends ReduxAction<AppState> {
  RequestStatusAction();

  @override
  Future<AppState?> reduce() async {
    // Ask for clock
    var clockMsg = Uint8Buffer();
    clockMsg.add(REQUEST_RTC_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, clockMsg);
    await Future.delayed(const Duration(milliseconds: 200));
    // Ask for outputs
    var outputsMsg = Uint8Buffer();
    outputsMsg.add(REQUEST_OUTPUTS_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, outputsMsg);
    await Future.delayed(const Duration(milliseconds: 200));
    return null;
  }
}
