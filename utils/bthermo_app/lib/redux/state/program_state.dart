import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';

class ProgramState {
  final int programId;
  final DateTime fromTime;
  final DateTime toTime;
  final double temperatureSetpoint;
  final OutputType output;
  final RepeatType repeat;

  ProgramState(
      {required this.programId,
      required this.fromTime,
      required this.toTime,
      required this.temperatureSetpoint,
      required this.output,
      required this.repeat});

  static ProgramState initialState() => ProgramState(
      programId: 0,
      fromTime: DateTime.now(),
      toTime: DateTime.now(),
      temperatureSetpoint: 0.0,
      output: OutputType.OUT1,
      repeat: RepeatType.NoRepeat);

  ProgramState copy(
          {int? programId,
          DateTime? fromTime,
          DateTime? toTime,
          double? temperatureSetpoint,
          OutputType? output,
          RepeatType? repeat}) =>
      ProgramState(
          programId: programId ?? this.programId,
          fromTime: fromTime ?? this.fromTime,
          toTime: toTime ?? this.toTime,
          temperatureSetpoint: temperatureSetpoint ?? this.temperatureSetpoint,
          output: output ?? this.output,
          repeat: repeat ?? this.repeat);
}
