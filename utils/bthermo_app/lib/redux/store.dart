import 'dart:async';

import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/constants.dart';
import 'package:bthermo_app/redux/actions/request_status_action.dart';
import 'package:bthermo_app/redux/actions/request_temperature_action.dart';
import 'package:flutter/material.dart';
import 'package:bthermo_app/redux/state/app_state.dart';

// App store
Store<AppState>? store;

// Navigator key
class NavigationService {
  static final GlobalKey<NavigatorState> navigatorKey =
      GlobalKey<NavigatorState>();
}

// Timers
Timer? statusTimer;
Timer? temperatureTimer;
bool statusActive = false;
bool tempActive = false;

void startStatusTimer() {
  statusActive = true;
  store!.dispatchAsync(RequestStatusAction());
  statusTimer = Timer.periodic(UPDATE_EVERY, (timer) {
    store!.dispatchAsync(RequestStatusAction());
  });
}

void cancelStatusTimer() {
  statusActive = false;
  statusTimer?.cancel();
}

void startTemperatureTimer() {
  tempActive = true;
  store!.dispatchAsync(RequestTemperatureAction());
  temperatureTimer = Timer.periodic(UPDATE_EVERY, (timer) {
    store!.dispatchAsync(RequestTemperatureAction());
  });
}

void cancelTemperatureTimer() {
  tempActive = false;
  temperatureTimer?.cancel();
}

void suspendTimers() {
  if (statusActive) {
    statusTimer?.cancel();
  }
  if (tempActive) {
    temperatureTimer?.cancel();
  }
}

void resumeTimers() {
  if (statusActive) {
    startStatusTimer();
  }
  if (tempActive) {
    startTemperatureTimer();
  }
}
