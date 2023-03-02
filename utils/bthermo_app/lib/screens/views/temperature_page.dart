import 'dart:math';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/actions/sync_clock_action.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:lite_rolling_switch/lite_rolling_switch.dart';
import 'package:provider/provider.dart';
import 'package:provider_for_redux/provider_for_redux.dart';
import 'package:syncfusion_flutter_charts/charts.dart';
import 'package:syncfusion_flutter_charts/sparkcharts.dart';
import 'package:syncfusion_flutter_gauges/gauges.dart';
import 'package:charts_flutter/flutter.dart' as charts;

import '../../constants.dart';
import '../../redux/state/app_state.dart';
import '../components/clock_view.dart';

class TemperaturePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
        child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              SizedBox(
                height: 40,
                child: Column(
                  children: [
                    Text(
                      'Temperature',
                      style: TextStyle(
                          fontWeight: FontWeight.w700,
                          color: CustomColors.primaryTextColor,
                          fontSize: 24),
                    )
                  ],
                ),
              ),
              ReduxSelector<AppState, dynamic>(
                selector: (context, state) => [
                  state.deviceTemperatureHistory,
                  state.deviceTemperatureOperation
                ],
                builder: (context, store, state, dispatch, model, child) {
                  var nonNullTemps =
                      state.deviceTemperatureHistory.where((t) => t != 0);
                  var lastTemp =
                      nonNullTemps.isNotEmpty ? nonNullTemps.last : 0.0;
                  var temps = state.deviceTemperatureHistory
                      .map((t) => (t * 10).round() / 10)
                      .toList();

                  var tempMin = 0;
                  var tempMax = 0;
                  if (temps.isNotEmpty) {
                    tempMin = temps.reduce(min).round();
                    tempMax = temps.reduce(max).round();
                  }

                  return Column(
                      mainAxisAlignment: MainAxisAlignment.spaceAround,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          lastTemp.toStringAsFixed(1),
                          style: TextStyle(
                              color: CustomColors.primaryTextColor,
                              fontSize: 64),
                        ),
                        SfRadialGauge(
                          axes: <RadialAxis>[
                            RadialAxis(
                              minimum: -20,
                              maximum: 51,
                              interval: 10,
                              radiusFactor: 0.55,
                              showFirstLabel: true,
                              showAxisLine: false,
                              labelOffset: 5,
                              useRangeColorForAxis: true,
                              axisLabelStyle: const GaugeTextStyle(
                                  fontWeight: FontWeight.bold, fontSize: 0),
                              ranges: <GaugeRange>[
                                GaugeRange(
                                    startValue: -20,
                                    endValue: -5,
                                    sizeUnit: GaugeSizeUnit.factor,
                                    color: Colors.red,
                                    endWidth: 0.03,
                                    startWidth: 0.03),
                                GaugeRange(
                                    startValue: -5,
                                    endValue: 10,
                                    sizeUnit: GaugeSizeUnit.factor,
                                    color: Colors.yellow,
                                    endWidth: 0.03,
                                    startWidth: 0.03),
                                GaugeRange(
                                    startValue: 10,
                                    endValue: 30,
                                    sizeUnit: GaugeSizeUnit.factor,
                                    color: Colors.green,
                                    endWidth: 0.03,
                                    startWidth: 0.03),
                                GaugeRange(
                                    startValue: 30,
                                    endValue: 40,
                                    sizeUnit: GaugeSizeUnit.factor,
                                    color: Colors.yellow,
                                    endWidth: 0.03,
                                    startWidth: 0.03),
                                GaugeRange(
                                    startValue: 40,
                                    endValue: 50,
                                    sizeUnit: GaugeSizeUnit.factor,
                                    color: Colors.red,
                                    endWidth: 0.03,
                                    startWidth: 0.03),
                              ],
                              annotations: const <GaugeAnnotation>[
                                GaugeAnnotation(
                                    widget: Text(
                                      '°C',
                                      style: TextStyle(
                                          fontSize: 17,
                                          fontWeight: FontWeight.w600),
                                    ),
                                    positionFactor: 0.9,
                                    angle: 90)
                              ],
                            ),
                            RadialAxis(
                                ticksPosition: ElementsPosition.outside,
                                labelsPosition: ElementsPosition.outside,
                                minorTicksPerInterval: 5,
                                axisLineStyle: const AxisLineStyle(
                                  thicknessUnit: GaugeSizeUnit.factor,
                                  thickness: 0.1,
                                ),
                                axisLabelStyle: const GaugeTextStyle(
                                    fontWeight: FontWeight.bold, fontSize: 16),
                                radiusFactor: 0.97,
                                majorTickStyle: const MajorTickStyle(
                                    length: 0.1,
                                    thickness: 2,
                                    lengthUnit: GaugeSizeUnit.factor),
                                minorTickStyle: const MinorTickStyle(
                                    length: 0.05,
                                    thickness: 1.5,
                                    lengthUnit: GaugeSizeUnit.factor),
                                minimum: -20,
                                maximum: 50,
                                interval: 5,
                                startAngle: 115,
                                endAngle: 65,
                                ranges: <GaugeRange>[
                                  GaugeRange(
                                      startValue: -20,
                                      endValue: 50,
                                      startWidth: 0.1,
                                      sizeUnit: GaugeSizeUnit.factor,
                                      endWidth: 0.1,
                                      gradient: const SweepGradient(
                                          stops: <double>[
                                            0.15,
                                            0.3,
                                            0.7,
                                            0.75,
                                            0.9
                                          ],
                                          colors: <Color>[
                                            Colors.red,
                                            Colors.yellow,
                                            Colors.green,
                                            Colors.yellow,
                                            Colors.red
                                          ]))
                                ],
                                pointers: <GaugePointer>[
                                  NeedlePointer(
                                      value: lastTemp,
                                      needleColor: Colors.black,
                                      tailStyle: const TailStyle(
                                          length: 0.18,
                                          width: 8,
                                          color: Colors.black,
                                          lengthUnit: GaugeSizeUnit.factor),
                                      needleLength: 0.68,
                                      needleStartWidth: 1,
                                      needleEndWidth: 8,
                                      knobStyle: const KnobStyle(
                                          knobRadius: 0.07,
                                          color: Colors.white,
                                          borderWidth: 0.05,
                                          borderColor: Colors.black),
                                      lengthUnit: GaugeSizeUnit.factor)
                                ]),
                          ],
                        ),
                        Container(
                            height: 300,
                            child: Padding(
                                padding: const EdgeInsets.all(20),
                                child: Align(
                                    alignment: Alignment.center,
                                    child: charts.LineChart(
                                      [
                                        charts.Series(
                                          id: 'Temperature',
                                          data: temps,
                                          domainFn: (datum, index) => index!,
                                          measureFn: (datum, index) =>
                                              temps[index!],
                                        )
                                      ],
                                      animate: false,
                                      defaultRenderer:
                                          charts.LineRendererConfig(
                                              includePoints: true),
                                      domainAxis: const charts.NumericAxisSpec(
                                        renderSpec: charts.NoneRenderSpec(),
                                      ),
                                      primaryMeasureAxis:
                                          charts.NumericAxisSpec(
                                              showAxisLine: true,
                                              renderSpec: charts.GridlineRendererSpec(
                                                  labelStyle: charts.TextStyleSpec(
                                                      color: charts.ColorUtil
                                                          .fromDartColor(
                                                              Colors.white)),
                                                  lineStyle: charts.LineStyleSpec(
                                                      color: charts.ColorUtil.fromDartColor(
                                                          const Color.fromARGB(
                                                              160,
                                                              255,
                                                              255,
                                                              255)))),
                                              tickProviderSpec:
                                                  charts.StaticNumericTickProviderSpec([
                                                for (double i =
                                                        tempMin.toDouble() - 1;
                                                    i <= tempMax.toDouble() + 1;
                                                    i += 0.5)
                                                  charts.TickSpec<num>(i)
                                              ])),
                                    )))),
                        Align(
                            alignment: Alignment.center,
                            child: Text(
                              "Operation Value: ${state.deviceTemperatureOperation.toStringAsFixed(2)}",
                              style: TextStyle(
                                  color: CustomColors.primaryTextColor,
                                  fontSize: 32),
                            ))
                      ]);
                },
              )
            ]));
  }
}
