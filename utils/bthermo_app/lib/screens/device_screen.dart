import 'package:bthermo_app/redux/enums/selected_screen.dart';
import 'package:bthermo_app/screens/components/vertical_menu.dart';
import 'package:bthermo_app/screens/views/programs_page.dart';
import 'package:bthermo_app/screens/views/status_page.dart';
import 'package:bthermo_app/screens/views/temperature_page.dart';
import 'package:flutter/material.dart';
import 'package:provider_for_redux/provider_for_redux.dart';

import '../constants.dart';
import '../redux/state/app_state.dart';

class DeviceScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return SafeArea(
        child: Scaffold(
            backgroundColor: CustomColors.pageBackgroundColor,
            body: Row(children: <Widget>[
              verticalMenu(),
              VerticalDivider(
                color: CustomColors.dividerColor,
                width: 1,
              ),
              Expanded(
                child: ReduxSelector<AppState, dynamic>(
                    selector: (context, state) => [state.screen],
                    builder: (context, store, state, dispatch, model, child) {
                      switch (state.screen) {
                        case SelectedScreen.Status:
                          {
                            return StatusPage();
                          }
                        case SelectedScreen.Temperature:
                          {
                            return TemperaturePage();
                          }
                        case SelectedScreen.Programs:
                          {
                            return ProgramsPage();
                          }
                      }
                    }),
              )
            ])));
  }
}
