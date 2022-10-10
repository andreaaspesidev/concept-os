import 'dart:math';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/actions/sync_clock_action.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:lite_rolling_switch/lite_rolling_switch.dart';
import 'package:provider/provider.dart';
import 'package:provider_for_redux/provider_for_redux.dart';

import '../../constants.dart';
import '../../redux/state/app_state.dart';
import '../components/clock_view.dart';

class StatusPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: <
            Widget>[
          SizedBox(
            height: 40,
            child: Column(
              children: [
                Text(
                  'Device Status',
                  style: TextStyle(
                      fontWeight: FontWeight.w700,
                      color: CustomColors.primaryTextColor,
                      fontSize: 24),
                )
              ],
            ),
          ),
          Column(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Column(
                  mainAxisAlignment: MainAxisAlignment.spaceAround,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    ReduxSelector<AppState, dynamic>(
                        selector: (context, state) => [state.deviceTime],
                        builder:
                            (context, store, state, dispatch, model, child) {
                          var formattedTime =
                              DateFormat('HH:mm').format(state.deviceTime);
                          var formattedDate =
                              DateFormat('EEE, d MMM').format(state.deviceTime);
                          return Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: <Widget>[
                              Text(
                                formattedTime,
                                style: TextStyle(
                                    color: CustomColors.primaryTextColor,
                                    fontSize: 64),
                              ),
                              Text(
                                formattedDate,
                                style: TextStyle(
                                    fontWeight: FontWeight.w300,
                                    color: CustomColors.primaryTextColor,
                                    fontSize: 20),
                              ),
                            ],
                          );
                        }),
                    Align(
                      alignment: Alignment.center,
                      child: Column(children: [
                        ClockView(
                          size: min(MediaQuery.of(context).size.width / 2,
                              MediaQuery.of(context).size.height / 2),
                        ),
                        Align(
                          alignment: Alignment.center,
                          child: InkWell(
                            onTap: () async =>
                                await Provider.of<Store<AppState>>(context,
                                        listen: false)
                                    .dispatchAsync(SyncClockAction()),
                            child: Container(
                              width: 200,
                              margin: const EdgeInsets.only(top: 20.0),
                              padding: const EdgeInsets.all(10.0),
                              decoration: const BoxDecoration(
                                  gradient: LinearGradient(
                                    colors: [
                                      Color(0xff2193b0),
                                      Color(0xff6dd5ed)
                                    ],
                                    begin: Alignment.centerLeft,
                                    end: Alignment.centerRight,
                                  ),
                                  borderRadius:
                                      BorderRadius.all(Radius.circular(12))),
                              child: Text("Sync Device Time",
                                  textAlign: TextAlign.center,
                                  style: Theme.of(context)
                                      .textTheme
                                      .titleLarge
                                      ?.copyWith(
                                          color:
                                              CustomColors.primaryTextColor)),
                            ),
                          ),
                        )
                      ]),
                    )
                  ]),
              Column(
                children: [
                  const SizedBox(height: 40),
                  ReduxSelector<AppState, dynamic>(
                      selector: (context, state) => [state.outputs],
                      builder: (context, store, state, dispatch, model, child) {
                        return Row(
                          mainAxisAlignment: MainAxisAlignment.spaceAround,
                          children: [
                            outputIndicator("OUT 1", state.outputs[0]),
                            outputIndicator("OUT 2", state.outputs[1]),
                            outputIndicator("OUT 3", state.outputs[2]),
                            outputIndicator("OUT 4", state.outputs[3]),
                          ],
                        );
                      })
                ],
              )
            ],
          )
        ]));
  }
}

Widget outputIndicator(String name, bool value) {
  return Column(
    children: [
      Text(name),
      Padding(
          padding: const EdgeInsets.symmetric(vertical: 5),
          child: SizedBox(
            width: 42,
            height: 42,
            child: DecoratedBox(
                decoration: BoxDecoration(
                    color: value ? Colors.green : Colors.red,
                    shape: BoxShape.circle)),
          ))
    ],
  );
}
