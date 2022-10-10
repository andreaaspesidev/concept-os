import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/actions/connect_action.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../constants.dart';
import '../redux/state/app_state.dart';

class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Stack(
      fit: StackFit.expand,
      children: [
        SafeArea(
            child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 20.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              const Spacer(flex: 1),
              const Text("bThermo",
                  style: TextStyle(fontSize: 40, color: Colors.white)),
              const Spacer(flex: 1),
              InkWell(
                onTap: () async =>
                    await Provider.of<Store<AppState>>(context, listen: false)
                        .dispatchAsync(ConnectAction()),
                child: Container(
                  width: 300,
                  margin: const EdgeInsets.only(top: 20.0 / 2),
                  padding: const EdgeInsets.all(20.0 * .75),
                  decoration: const BoxDecoration(
                      gradient: LinearGradient(
                        colors: [Color(0xff2193b0), Color(0xff6dd5ed)],
                        begin: Alignment.centerLeft,
                        end: Alignment.centerRight,
                      ),
                      borderRadius: BorderRadius.all(Radius.circular(12))),
                  child: Text("Connect",
                      textAlign: TextAlign.center,
                      style: Theme.of(context)
                          .textTheme
                          .titleLarge
                          ?.copyWith(color: CustomColors.primaryTextColor)),
                ),
              ),
              const Spacer(flex: 1),
              const Text("Developed by Andrea Aspesi",
                  textAlign: TextAlign.center,
                  style: TextStyle(fontSize: 15, color: Colors.white)),
            ],
          ),
        ))
      ],
    ));
  }
}
