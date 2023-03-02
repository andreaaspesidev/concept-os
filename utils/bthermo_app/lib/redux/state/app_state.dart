import 'package:bthermo_app/redux/enums/message_type.dart';
import 'package:bthermo_app/redux/enums/selected_screen.dart';
import 'package:bthermo_app/redux/state/program_state.dart';

class AppState {
  final bool connected;
  final DateTime deviceTime;
  final List<double> deviceTemperatureHistory;
  final double deviceTemperatureOperation;
  final MessageType lastMsg;
  final List<ProgramState> programs;
  final List<bool> outputs;

  final SelectedScreen screen;

  AppState(
      {required this.connected,
      required this.deviceTime,
      required this.deviceTemperatureHistory,
      required this.deviceTemperatureOperation,
      required this.lastMsg,
      required this.programs,
      required this.outputs,
      required this.screen});

  static AppState initialState() => AppState(
      connected: false,
      deviceTime: DateTime.now(),
      deviceTemperatureHistory: List.empty(),
      deviceTemperatureOperation: 0.0,
      programs: List.empty(),
      outputs: List.from([false, false, false, false]),
      lastMsg: MessageType.None,
      screen: SelectedScreen.Status);

  AppState copy(
          {bool? connected,
          DateTime? deviceTime,
          List<double>? deviceTemperatureHistory,
          double? deviceTemperatureOperation,
          MessageType? lastMsg,
          List<ProgramState>? programs,
          List<bool>? outputs,
          SelectedScreen? screen}) =>
      AppState(
          connected: connected ?? this.connected,
          deviceTime: deviceTime ?? this.deviceTime,
          deviceTemperatureHistory:
              deviceTemperatureHistory ?? this.deviceTemperatureHistory,
          deviceTemperatureOperation:
              deviceTemperatureOperation ?? this.deviceTemperatureOperation,
          lastMsg: lastMsg ?? this.lastMsg,
          programs: programs ?? this.programs,
          outputs: outputs ?? this.outputs,
          screen: screen ?? this.screen);
}
