import 'package:bthermo_app/redux/actions/change_screen_action.dart';
import 'package:bthermo_app/redux/enums/selected_screen.dart';
import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:provider_for_redux/provider_for_redux.dart';

import '../../constants.dart';
import '../../redux/state/app_state.dart';

Widget verticalMenu() {
  return ReduxSelector<AppState, dynamic>(
      selector: (context, state) => [state.screen],
      builder: (context, store, state, dispatch, model, child) => Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              menuItem("Status", FontAwesomeIcons.house, () {
                store.dispatch(
                    ChangeScreenAction(screen: SelectedScreen.Status));
              }, state.screen == SelectedScreen.Status),
              menuItem("Temp.", FontAwesomeIcons.temperatureThreeQuarters, () {
                store.dispatch(
                    ChangeScreenAction(screen: SelectedScreen.Temperature));
              }, state.screen == SelectedScreen.Temperature),
              menuItem("Programs", FontAwesomeIcons.listCheck, () {
                store.dispatch(
                    ChangeScreenAction(screen: SelectedScreen.Programs));
              }, state.screen == SelectedScreen.Programs)
            ],
          ));
}

Widget menuItem(
    String? text, IconData icon, VoidCallback onPressed, bool isSelected) {
  return MaterialButton(
    shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.only(topRight: Radius.circular(32))),
    padding: const EdgeInsets.symmetric(vertical: 16.0, horizontal: 0),
    color: isSelected
        ? CustomColors.menuBackgroundColor
        : CustomColors.pageBackgroundColor,
    onPressed: () {
      onPressed.call();
    },
    child: Column(
      children: <Widget>[
        FaIcon(icon),
        const SizedBox(height: 16),
        Text(
          text ?? '',
          style: TextStyle(color: CustomColors.primaryTextColor, fontSize: 14),
        ),
      ],
    ),
  );
}
