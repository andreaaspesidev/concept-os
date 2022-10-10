import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/actions/load_programs_action.dart';
import 'package:bthermo_app/redux/enums/selected_screen.dart';
import 'package:bthermo_app/redux/store.dart';

import '../state/app_state.dart';

class ChangeScreenAction extends ReduxAction<AppState> {
  final SelectedScreen screen;
  ChangeScreenAction({required this.screen});

  @override
  Future<AppState> reduce() async {
    // Activate/Deactivate things based on screen transition
    switch (state.screen) {
      case SelectedScreen.Status:
        {
          // Cancel timer
          cancelStatusTimer();
          break;
        }
      case SelectedScreen.Temperature:
        {
          // Cancel timer
          cancelTemperatureTimer();
          break;
        }
      case SelectedScreen.Programs:
        // Nothing to do here
        break;
    }
    switch (screen) {
      case SelectedScreen.Status:
        {
          startStatusTimer();
          break;
        }
      case SelectedScreen.Temperature:
        {
          startTemperatureTimer();
          break;
        }
      case SelectedScreen.Programs:
        {
          await dispatchAsync(LoadProgramsAction());
        }
    }
    // Change state
    return state.copy(screen: screen);
  }
}
