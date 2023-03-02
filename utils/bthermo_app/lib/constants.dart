// MQTT constants
import 'package:flutter/material.dart';

const MQTT_SERVER = '192.168.0.3';
const MQTT_SERVER_PORT = 1883;
const MQTT_USER = 'mqtt';
const MQTT_PASSWD = 'mqtt';

// THERMO constants
const THERMO_IN_TOPIC = "bthermo/11/in";
const THERMO_OUT_TOPIC = "bthermo/11/out";

const UPDATE_EVERY = Duration(seconds: 1);
const MAX_PROGRAMS = 16;

// Commands
const REQUEST_RTC_CMD = 0x63;
const REQUEST_OUTPUTS_CMD = 0x6F;
const REQUEST_TEMP_CMD = 0x74;
const SET_RTC_CMD = 0x73;
const ADD_PROGRAM_CMD = 0x6E;
const REMOVE_PROGRAM_CMD = 0x64;
const REQUEST_PROGRAMS_CMD = 0x70;

/**
 * Graphics
 */

class CustomColors {
  static Color primaryTextColor = Colors.white;
  static Color dividerColor = Colors.white54;
  static Color pageBackgroundColor = Color(0xFF2D2F41);
  static Color menuBackgroundColor = Color(0xFF242634);

  static Color clockBG = Color(0xFF444974);
  static Color clockOutline = Color(0xFFEAECFF);
  static Color? secHandColor = Colors.orange[300];
  static Color minHandStatColor = Color(0xFF748EF6);
  static Color minHandEndColor = Color(0xFF77DDFF);
  static Color hourHandStatColor = Color(0xFFC279FB);
  static Color hourHandEndColor = Color(0xFFEA74AB);
}

class GradientColors {
  final List<Color> colors;
  GradientColors(this.colors);

  static List<Color> sky = [Color(0xFF6448FE), Color(0xFF5FC6FF)];
  static List<Color> sunset = [Color(0xFFFE6197), Color(0xFFFFB463)];
  static List<Color> sea = [Color(0xFF61A3FE), Color(0xFF63FFD5)];
  static List<Color> mango = [Color(0xFFFFA738), Color(0xFFFFE130)];
  static List<Color> fire = [Color(0xFFFF5DCD), Color(0xFFFF8484)];
}

class GradientTemplate {
  static List<GradientColors> gradientTemplate = [
    GradientColors(GradientColors.sky),
    GradientColors(GradientColors.sunset),
    GradientColors(GradientColors.sea),
    GradientColors(GradientColors.mango),
    GradientColors(GradientColors.fire),
  ];
}
