import 'dart:typed_data';

import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:typed_data/typed_buffers.dart';

Uint8Buffer dateToBytes(DateTime datetime) {
  Uint8Buffer bytes = Uint8Buffer();
  bytes.add(datetime.second);
  bytes.add(datetime.minute);
  bytes.add(datetime.hour);
  bytes.add(datetime.weekday);
  bytes.add(datetime.day);
  bytes.add(datetime.month);
  bytes.add(datetime.year & 0xFF);
  bytes.add(datetime.year >> 8);
  return bytes;
}

Uint8List doubleToBytes(double setpoint) {
  ByteData bytes = ByteData(4);
  bytes.setFloat32(0, setpoint, Endian.little);
  return bytes.buffer.asUint8List();
}

Uint8Buffer outputToBytes(OutputType outputType) {
  Uint8Buffer bytes = Uint8Buffer();
  switch (outputType) {
    case OutputType.OUT1:
      {
        bytes.add(1);
        break;
      }
    case OutputType.OUT2:
      {
        bytes.add(2);
        break;
      }
    case OutputType.OUT3:
      {
        bytes.add(3);
        break;
      }
    case OutputType.OUT4:
      {
        bytes.add(4);
        break;
      }
  }
  return bytes;
}

Uint8Buffer repeatToBytes(RepeatType repeatType) {
  Uint8Buffer bytes = Uint8Buffer();
  switch (repeatType) {
    case RepeatType.NoRepeat:
      {
        bytes.add(0);
        break;
      }
    case RepeatType.EveryDay:
      {
        bytes.add(1);
        break;
      }
    case RepeatType.EveryWeek:
      {
        bytes.add(2);
        break;
      }
    case RepeatType.EveryMonth:
      {
        bytes.add(3);
        break;
      }
    case RepeatType.EveryYear:
      {
        bytes.add(4);
        break;
      }
  }
  return bytes;
}
