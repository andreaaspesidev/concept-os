import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_buffers.dart';

import '../../constants.dart';
import '../../integrations/mqtt.dart';
import '../state/app_state.dart';

class RemovedProgramAction extends ReduxAction<AppState> {
  final int programId;

  RemovedProgramAction({required this.programId});

  @override
  Future<AppState?> reduce() async {
    suspendTimers();
    // Remove program
    var requestPkt = Uint8Buffer();
    requestPkt.add(REMOVE_PROGRAM_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, requestPkt);
    requestPkt.clear();
    await Future.delayed(const Duration(milliseconds: 100));
    requestPkt.add(programId & 0xFF);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, requestPkt);
    resumeTimers();
    return null;
  }
}
