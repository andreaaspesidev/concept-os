import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_buffers.dart';

import '../../constants.dart';
import '../../integrations/mqtt.dart';
import '../state/app_state.dart';

class SyncClockAction extends ReduxAction<AppState> {
  @override
  Future<AppState?> reduce() async {
    suspendTimers();
    // Ask for clock
    var date = DateTime.now();
    var dateMsg = Uint8Buffer();
    dateMsg.add(SET_RTC_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, dateMsg);
    dateMsg.clear();
    await Future.delayed(const Duration(milliseconds: 100));
    dateMsg.add(date.day);
    dateMsg.add(date.month);
    dateMsg.add(date.year & 0xFF);
    dateMsg.add(date.year >> 8);
    dateMsg.add(date.weekday);
    dateMsg.add(date.hour);
    dateMsg.add(date.minute);
    dateMsg.add(date.second);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, dateMsg);
    resumeTimers();
    return null;
  }
}
