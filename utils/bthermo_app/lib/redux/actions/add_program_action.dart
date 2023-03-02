import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:bthermo_app/utils.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:typed_data/typed_buffers.dart';

import '../../constants.dart';
import '../../integrations/mqtt.dart';
import '../state/app_state.dart';

class AddProgramAction extends ReduxAction<AppState> {
  final DateTime fromDate;
  final DateTime toDate;
  final double temperatureSetpoint;
  final RepeatType repeatType;
  final OutputType outputType;

  AddProgramAction(
      {required this.fromDate,
      required this.toDate,
      required this.temperatureSetpoint,
      required this.repeatType,
      required this.outputType});

  @override
  Future<AppState?> reduce() async {
    suspendTimers();
    // Set program
    var requestPkt = Uint8Buffer();
    requestPkt.add(ADD_PROGRAM_CMD);
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, requestPkt);
    requestPkt.clear();
    await Future.delayed(const Duration(milliseconds: 100));
    // Add from date
    requestPkt.addAll(dateToBytes(fromDate));
    // Add to date
    requestPkt.addAll(dateToBytes(toDate));
    // Add setpoint
    requestPkt.addAll(doubleToBytes(temperatureSetpoint));
    // Add output type
    requestPkt.addAll(outputToBytes(outputType));
    // Add repeat type
    requestPkt.addAll(repeatToBytes(repeatType));
    // Send data
    client.publishMessage(THERMO_IN_TOPIC, MqttQos.exactlyOnce, requestPkt);
    resumeTimers();
    return null;
  }
}
