import 'dart:math';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/actions/add_program_action.dart';
import 'package:bthermo_app/redux/actions/load_programs_action.dart';
import 'package:bthermo_app/redux/actions/remove_program_action.dart';
import 'package:bthermo_app/redux/enums/output_type.dart';
import 'package:bthermo_app/redux/enums/repeat_type.dart';
import 'package:bthermo_app/redux/state/program_state.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:numberpicker/numberpicker.dart';
import 'package:provider/provider.dart';
import 'package:provider_for_redux/provider_for_redux.dart';
import '../../constants.dart';
import '../../redux/state/app_state.dart';

class ProgramsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ReduxSelector<AppState, dynamic>(
        selector: (context, state) => [state.programs],
        builder: (context, store, state, dispatch, model, child) {
          List<Widget> programs = state.programs.map((p) {
            return programElement(
                GradientTemplate
                    .gradientTemplate[GradientTemplate.gradientTemplate.length %
                        (p.programId + 1)]
                    .colors,
                p,
                context);
          }).toList();
          return Scaffold(
              floatingActionButton: state.programs.length < MAX_PROGRAMS
                  ? FloatingActionButton(
                      child: const Icon(Icons.add),
                      tooltip: "Add Program",
                      onPressed: () async {
                        await showDialog(
                            context: context,
                            builder: (BuildContext context) {
                              return DialogAddProgram();
                            });
                      })
                  : null,
              body: Container(
                  padding:
                      const EdgeInsets.symmetric(horizontal: 10, vertical: 20),
                  child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: <Widget>[
                        SizedBox(
                          height: 40,
                          child: Column(
                            children: [
                              Text(
                                'Programs',
                                style: TextStyle(
                                    fontWeight: FontWeight.w700,
                                    color: CustomColors.primaryTextColor,
                                    fontSize: 24),
                              )
                            ],
                          ),
                        ),
                        Expanded(
                            child: ListView(
                          children: programs,
                        ))
                      ])));
        });
  }
}

Widget programElement(
    List<Color> colors, ProgramState program, BuildContext context) {
  var fontSize = min(MediaQuery.of(context).size.width / 30, 20.0);
  var dateDiv = MediaQuery.of(context).size.width > 500
      ? Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              'From: ${DateFormat("HH:mm:ss of d/M/y").format(program.fromTime)}',
              style: TextStyle(
                  color: Colors.white,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700),
            ),
            Text(
              'To: ${DateFormat("HH:mm:ss of d/M/y").format(program.toTime)}',
              style: TextStyle(
                  color: Colors.white,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700),
            ),
          ],
        )
      : Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'From: ${DateFormat("HH:mm:ss of d/M/y").format(program.fromTime)}',
              style: TextStyle(
                  color: Colors.white,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700),
            ),
            const SizedBox(height: 5),
            Text(
              'To: ${DateFormat("HH:mm:ss of d/M/y").format(program.toTime)}',
              style: TextStyle(
                  color: Colors.white,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700),
            ),
          ],
        );
  return Container(
      margin: const EdgeInsets.only(bottom: 20),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 5),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: colors,
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
        ),
        borderRadius: const BorderRadius.all(Radius.circular(24)),
      ),
      child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: <Widget>[
                Row(
                  children: <Widget>[
                    const Icon(
                      Icons.label,
                      color: Colors.white,
                      size: 24,
                    ),
                    const SizedBox(width: 8),
                    Text(
                      'ID: ${program.programId}',
                      style: const TextStyle(color: Colors.white),
                    ),
                  ],
                )
              ],
            ),
            dateDiv,
            const SizedBox(height: 10),
            Align(
                alignment: Alignment.center,
                child: Text(
                  'Temperature: ${program.temperatureSetpoint.toStringAsFixed(1)}',
                  style: TextStyle(
                      color: Colors.white,
                      fontSize: fontSize + 5,
                      fontWeight: FontWeight.w700),
                )),
            const SizedBox(height: 10),
            Text(
              'Repeat: ${program.repeat.name}',
              style: TextStyle(
                  color: Colors.white,
                  fontSize: fontSize,
                  fontWeight: FontWeight.w700),
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: <Widget>[
                Text(
                  'Output: ${program.output.name}',
                  style: TextStyle(
                      color: Colors.white,
                      fontSize: fontSize,
                      fontWeight: FontWeight.w700),
                ),
                IconButton(
                    icon: const Icon(Icons.delete),
                    color: Colors.white,
                    onPressed: () async {
                      await Provider.of<Store<AppState>>(context, listen: false)
                          .dispatchAsync(RemovedProgramAction(
                              programId: program.programId));
                      await Future.delayed(const Duration(milliseconds: 100));
                      await Provider.of<Store<AppState>>(context, listen: false)
                          .dispatchAsync(LoadProgramsAction());
                    }),
              ],
            )
          ]));
}

class DialogAddProgram extends StatefulWidget {
  @override
  _DialogAddProgramState createState() => _DialogAddProgramState();
}

class _DialogAddProgramState extends State<DialogAddProgram> {
  DateTime _fromDate = DateTime.now();
  DateTime _toDate = DateTime.now().add(const Duration(hours: 1));
  RepeatType _repeatType = RepeatType.NoRepeat;
  OutputType _outputType = OutputType.OUT1;
  double _setpoint = 20.0;

  Future<void> _selectFromDate(BuildContext context) async {
    final DateTime? picked = await showDatePicker(
        context: context,
        initialDate: _fromDate,
        firstDate: DateTime.now(),
        lastDate: DateTime.now().add(const Duration(days: 365)));
    final TimeOfDay? timePicked = await showTimePicker(
        context: context, initialTime: TimeOfDay.fromDateTime(_fromDate));
    if (picked != null && picked != _fromDate) {
      setState(() {
        _fromDate = picked;
      });
    }
    if (timePicked != null) {
      setState(() {
        _fromDate = _fromDate.copyWith(
            hour: timePicked.hour, minute: timePicked.minute, second: 0);
      });
    }
  }

  Future<void> _selectToDate(BuildContext context) async {
    final DateTime? picked = await showDatePicker(
        context: context,
        initialDate: _fromDate.compareTo(_toDate) > 0 ? _fromDate : _toDate,
        firstDate: _fromDate,
        lastDate: _fromDate.add(const Duration(days: 365)));
    final TimeOfDay? timePicked = await showTimePicker(
        context: context,
        initialTime: TimeOfDay.fromDateTime(
            _fromDate.compareTo(_toDate) > 0 ? _fromDate : _toDate));
    if (picked != null && picked != _toDate) {
      setState(() {
        _toDate = picked;
      });
    }
    if (timePicked != null) {
      setState(() {
        _toDate = _toDate.copyWith(
            hour: max(timePicked.hour, _fromDate.hour),
            minute: max(timePicked.minute, _fromDate.minute),
            second: 0);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      elevation: 16,
      child: Container(
        width: max(MediaQuery.of(context).size.width * 0.5, 600),
        child: Column(mainAxisSize: MainAxisSize.min, children: [
          ListView(
            shrinkWrap: true,
            children: [
              const Padding(
                padding: EdgeInsets.symmetric(vertical: 10.0, horizontal: 10.0),
                child: Center(
                    child: Text(
                  "Add Program",
                  style: TextStyle(fontSize: 20.0),
                )),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(
                    vertical: 10.0, horizontal: 30.0),
                child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text(
                        "From: ",
                        style: TextStyle(fontSize: 15),
                      ),
                      Text(DateFormat("HH:mm:ss of d/M/y").format(_fromDate)),
                      MaterialButton(
                        minWidth: 50,
                        padding: const EdgeInsets.all(10.0),
                        onPressed: () => _selectFromDate(context),
                        child: const Icon(Icons.date_range),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(10.0),
                        ),
                      ),
                    ]),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(
                    vertical: 10.0, horizontal: 30.0),
                child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text(
                        "To: ",
                        style: TextStyle(fontSize: 15),
                      ),
                      Text(DateFormat("HH:mm:ss of d/M/y").format(_toDate)),
                      MaterialButton(
                        minWidth: 50,
                        padding: const EdgeInsets.all(10.0),
                        onPressed: () => _selectToDate(context),
                        child: const Icon(Icons.date_range),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(10.0),
                        ),
                      ),
                    ]),
              ),
              Padding(
                  padding: const EdgeInsets.symmetric(
                      vertical: 10.0, horizontal: 30.0),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.start,
                    children: [
                      const Text(
                        "Temperature: ",
                        style: TextStyle(fontSize: 15),
                      ),
                      DecimalNumberPicker(
                        value: _setpoint,
                        axis: Axis.vertical,
                        itemWidth: 40.0,
                        itemHeight: 50.0,
                        minValue: 0,
                        maxValue: 50,
                        decimalPlaces: 1,
                        onChanged: (value) {
                          setState(() {
                            _setpoint = value;
                          });
                        },
                      )
                    ],
                  )),
              Padding(
                  padding: const EdgeInsets.symmetric(
                      vertical: 10.0, horizontal: 30.0),
                  child: DropdownButton<RepeatType>(
                    hint: const Text("Select Repetition"),
                    isExpanded: true,
                    value: _repeatType,
                    items: RepeatType.values
                        .map((e) =>
                            DropdownMenuItem(value: e, child: Text(e.name)))
                        .toList(),
                    onChanged: (value) {
                      setState(() {
                        _repeatType = value!;
                      });
                    },
                  )),
              Padding(
                  padding: const EdgeInsets.symmetric(
                      vertical: 10.0, horizontal: 30.0),
                  child: DropdownButton<OutputType>(
                    hint: const Text("Select Output"),
                    isExpanded: true,
                    value: _outputType,
                    items: OutputType.values
                        .map((e) =>
                            DropdownMenuItem(value: e, child: Text(e.name)))
                        .toList(),
                    onChanged: (value) {
                      setState(() {
                        _outputType = value!;
                      });
                    },
                  )),
              Align(
                  alignment: Alignment.center,
                  child: Padding(
                    padding:
                        EdgeInsets.symmetric(vertical: 10.0, horizontal: 30.0),
                    child: InkWell(
                      onTap: () async {
                        await Provider.of<Store<AppState>>(context,
                                listen: false)
                            .dispatchAsync(AddProgramAction(
                                fromDate: _fromDate,
                                toDate: _toDate,
                                temperatureSetpoint: _setpoint,
                                repeatType: _repeatType,
                                outputType: _outputType));
                        await Future.delayed(const Duration(milliseconds: 100));
                        await Provider.of<Store<AppState>>(context,
                                listen: false)
                            .dispatchAsync(LoadProgramsAction());
                        // Wait before closing
                        Navigator.pop(context);
                      },
                      child: Container(
                        width: 200,
                        margin: const EdgeInsets.only(top: 20.0),
                        padding: const EdgeInsets.all(10.0),
                        decoration: const BoxDecoration(
                            gradient: LinearGradient(
                              colors: [Color(0xff2193b0), Color(0xff6dd5ed)],
                              begin: Alignment.centerLeft,
                              end: Alignment.centerRight,
                            ),
                            borderRadius:
                                BorderRadius.all(Radius.circular(12))),
                        child: Text("Add Program",
                            textAlign: TextAlign.center,
                            style: Theme.of(context)
                                .textTheme
                                .titleLarge
                                ?.copyWith(
                                    color: CustomColors.primaryTextColor)),
                      ),
                    ),
                  ))
            ],
          )
        ]),
      ),
    );
  }
}

extension MyDateUtils on DateTime {
  DateTime copyWith({
    int? year,
    int? month,
    int? day,
    int? hour,
    int? minute,
    int? second,
    int? millisecond,
    int? microsecond,
  }) {
    return DateTime(
      year ?? this.year,
      month ?? this.month,
      day ?? this.day,
      hour ?? this.hour,
      minute ?? this.minute,
      second ?? this.second,
      millisecond ?? this.millisecond,
      microsecond ?? this.microsecond,
    );
  }
}
