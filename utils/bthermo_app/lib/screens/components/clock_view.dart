import 'dart:math';

import 'package:flutter/material.dart';
import 'package:provider_for_redux/provider_for_redux.dart';

import '../../constants.dart';
import '../../redux/state/app_state.dart';

class ClockView extends StatelessWidget {
  final double size;

  ClockView({required this.size});

  @override
  Widget build(BuildContext context) {
    return Align(
      alignment: Alignment.topCenter,
      child: SizedBox(
          width: size,
          height: size,
          child: Transform.rotate(
            angle: -pi / 2,
            child: ReduxSelector<AppState, dynamic>(
              selector: (context, state) => [state.deviceTime],
              builder: (context, store, state, dispatch, model, child) =>
                  CustomPaint(
                      painter: ClockPainter(
                          hour: state.deviceTime.hour,
                          minute: state.deviceTime.minute,
                          second: state.deviceTime.second)),
            ),
          )),
    );
  }
}

class ClockPainter extends CustomPainter {
  final int hour;
  final int minute;
  final int second;

  ClockPainter(
      {required this.hour, required this.minute, required this.second});

  //60sec - 360, 1 sec - 6degrees
  //60min - 360, 1 min - 6degrees
  //12hours - 360, 1 hour - 30degrees, 60min - 30degrees, 1 min - 0.5degrees

  @override
  void paint(Canvas canvas, Size size) {
    var centerX = size.width / 2;
    var centerY = size.height / 2;
    var center = Offset(centerX, centerY);
    var radius = min(centerX, centerY);

    var fillBrush = Paint()..color = CustomColors.clockBG;
    var outlineBrush = Paint()
      ..color = CustomColors.clockOutline
      ..style = PaintingStyle.stroke
      ..strokeWidth = size.width / 20;
    var centerDotBrush = Paint()..color = CustomColors.clockOutline;

    var secHandBrush = Paint()
      ..color = CustomColors.secHandColor!
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = size.width / 60;

    var minHandBrush = Paint()
      ..shader = RadialGradient(colors: [
        CustomColors.minHandStatColor,
        CustomColors.minHandEndColor
      ]).createShader(Rect.fromCircle(center: center, radius: radius))
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = size.width / 30;

    var hourHandBrush = Paint()
      ..shader = RadialGradient(colors: [
        CustomColors.hourHandStatColor,
        CustomColors.hourHandEndColor
      ]).createShader(Rect.fromCircle(center: center, radius: radius))
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = size.width / 24;

    var dashBrush = Paint()
      ..color = CustomColors.clockOutline
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    canvas.drawCircle(center, radius * 0.75, fillBrush);
    canvas.drawCircle(center, radius * 0.75, outlineBrush);

    var hourHandX =
        centerX + radius * 0.4 * cos((hour * 30 + minute * 0.5) * pi / 180);
    var hourHandY =
        centerY + radius * 0.4 * sin((hour * 30 + minute * 0.5) * pi / 180);
    canvas.drawLine(center, Offset(hourHandX, hourHandY), hourHandBrush);

    var minHandX = centerX + radius * 0.6 * cos(minute * 6 * pi / 180);
    var minHandY = centerY + radius * 0.6 * sin(minute * 6 * pi / 180);
    canvas.drawLine(center, Offset(minHandX, minHandY), minHandBrush);

    var secHandX = centerX + radius * 0.6 * cos(second * 6 * pi / 180);
    var secHandY = centerY + radius * 0.6 * sin(second * 6 * pi / 180);
    canvas.drawLine(center, Offset(secHandX, secHandY), secHandBrush);

    canvas.drawCircle(center, radius * 0.12, centerDotBrush);

    var outerRadius = radius;
    var innerRadius = radius * 0.9;
    for (var i = 0; i < 360; i += 12) {
      var x1 = centerX + outerRadius * cos(i * pi / 180);
      var y1 = centerY + outerRadius * sin(i * pi / 180);

      var x2 = centerX + innerRadius * cos(i * pi / 180);
      var y2 = centerY + innerRadius * sin(i * pi / 180);
      canvas.drawLine(Offset(x1, y1), Offset(x2, y2), dashBrush);
    }
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return true;
  }
}
