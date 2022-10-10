import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_buffers.dart';

import '../../constants.dart';
import '../../integrations/mqtt.dart';
import '../state/app_state.dart';

class LoadProgramsAction extends ReduxAction<AppState> {
  @override
  Future<AppState?> reduce() async {
    suspendTimers();
    // Ask for programs
    var requestPkt = Uint8Buffer();
    requestPkt.add(REQUEST_PROGRAMS_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, requestPkt);
    resumeTimers();
    return null;
  }
}
