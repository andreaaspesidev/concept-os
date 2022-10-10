import 'package:async_redux/async_redux.dart';

import '../state/app_state.dart';

class OnDisconnectEvent extends ReduxAction<AppState> {
  OnDisconnectEvent();

  @override
  AppState reduce() {
    // Clear all state
    return AppState.initialState();
  }

  @override
  void after() {
    // Navigate to home page
    dispatch(NavigateAction.pushNamedAndRemoveAll("/home"));
  }
}
