import 'dart:typed_data';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/constants.dart';
import 'package:bthermo_app/redux/enums/message_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:bthermo_app/redux/state/app_state.dart';
import 'package:bthermo_app/redux/state/program_state.dart';
import 'package:typed_data/typed_buffers.dart';

import '../enums/output_type.dart';

class OnMessageEvent extends ReduxAction<AppState> {
  final Uint8List dataBytes;

  OnMessageEvent({required this.dataBytes});

  @override
  AppState? reduce() {
    // Decode message payload
    if (dataBytes.isEmpty) {
      return null; // Ignore
    }
    int dataType = dataBytes[0];
    if (dataType == REQUEST_RTC_CMD) {
      // The following bytes represent a TimeStamp structure
      var date = getDateFrom(dataBytes, 1);
      return state.copy(deviceTime: date);
    } else if (dataType == REQUEST_OUTPUTS_CMD) {
      var out1 = dataBytes[1] > 0;
      var out2 = dataBytes[2] > 0;
      var out3 = dataBytes[3] > 0;
      var out4 = dataBytes[4] > 0;
      return state.copy(outputs: List.from([out1, out2, out3, out4]));
    } else if (dataType == REQUEST_TEMP_CMD) {
      var numTemps = 16;
      List<double> history =
          [for (var i = 0; i < numTemps; i++) i].map((pIndex) {
        // Start addr
        var startAddr = 1 + pIndex * 4;
        var temperature =
            dataBytes.buffer.asByteData().getFloat32(startAddr, Endian.little);
        return temperature;
      }).toList();
      double operation =
          dataBytes.buffer.asByteData().getFloat32(1 + 16 * 4, Endian.little);
      return state.copy(
          deviceTemperatureHistory: history,
          deviceTemperatureOperation: operation);
    } else if (dataType == SET_RTC_CMD) {
      var success = dataBytes[1] == 0;
      if (success) {
        print("RTC set success!");
      } else {
        print("RTC set failed!");
      }
    } else if (dataType == ADD_PROGRAM_CMD) {
      var result = dataBytes[1];
      if (result == 0x00) {
        print("Program add success!");
      } else if (result == 0x01) {
        print("Invalid arguments!");
      } else if (result == 0x02) {
        print("Program add failed!");
      }
    } else if (dataType == REMOVE_PROGRAM_CMD) {
      var success = dataBytes[1] == 0;
      if (success) {
        print("Program remove success!");
      } else {
        print("Program remove failed!");
      }
    } else if (dataType == REQUEST_PROGRAMS_CMD) {
      int sizeOfTime = 8;
      int sizeOfProgram = 1 + sizeOfTime * 2 + 4 + 1 + 1;
      int numPrograms = (dataBytes.length - 1) ~/ sizeOfProgram;
      List<ProgramState> programs =
          [for (var i = 0; i < numPrograms; i++) i].map((pIndex) {
        // Start addr
        var startAddr = 1 + pIndex * sizeOfProgram;
        // Get a program at this index
        var programId = dataBytes[startAddr + 0];
        var fromTime = getDateFrom(dataBytes, startAddr + 1);
        var toTime = getDateFrom(dataBytes, startAddr + 1 + sizeOfTime);
        var setpoint = dataBytes.buffer
            .asByteData()
            .getFloat32(startAddr + 1 + 2 * sizeOfTime, Endian.little);
        var output =
            outputFromBytes(dataBytes, startAddr + 1 + 2 * sizeOfTime + 4);
        var repeat =
            repeatFromBytes(dataBytes, startAddr + 1 + 2 * sizeOfTime + 4 + 1);
        return ProgramState(
            programId: programId,
            fromTime: fromTime,
            toTime: toTime,
            temperatureSetpoint: setpoint,
            output: output,
            repeat: repeat);
      }).toList();
      return state.copy(programs: programs);
    }
    return null;
  }
}

DateTime getDateFrom(Uint8List bytes, int offset) {
  var sec = bytes[offset];
  var min = bytes[offset + 1];
  var hour = bytes[offset + 2];
  //var week_day = bytes[offset +3];
  var day = bytes[offset + 4];
  var month = bytes[offset + 5];
  var year = bytes[offset + 7] << 8 | bytes[offset + 6];
  return DateTime(year, month, day, hour, min, sec);
}

OutputType outputFromBytes(Uint8List bytes, int offset) {
  switch (bytes[offset]) {
    case 0x01:
      {
        return OutputType.OUT1;
      }
    case 0x02:
      {
        return OutputType.OUT2;
      }
    case 0x03:
      {
        return OutputType.OUT3;
      }
    case 0x04:
      {
        return OutputType.OUT4;
      }
  }
  throw Error();
}

RepeatType repeatFromBytes(Uint8List bytes, int offset) {
  switch (bytes[offset]) {
    case 0x00:
      {
        return RepeatType.NoRepeat;
      }
    case 0x01:
      {
        return RepeatType.EveryDay;
      }
    case 0x02:
      {
        return RepeatType.EveryWeek;
      }
    case 0x03:
      {
        return RepeatType.EveryMonth;
      }
    case 0x04:
      {
        return RepeatType.EveryYear;
      }
  }
  throw Error();
}
