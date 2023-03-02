import 'package:async_redux/async_redux.dart';
import 'package:bthermo_app/redux/state/app_state.dart';
import 'package:bthermo_app/redux/store.dart';
import 'package:bthermo_app/routes.dart';
import 'package:flutter/material.dart';
import 'package:provider_for_redux/provider_for_redux.dart';

void main() async {
  // Wait to the binding context to start. If not, firebase will throw an error
  WidgetsFlutterBinding.ensureInitialized();
  // Create store
  store = Store<AppState>(initialState: AppState.initialState());
  // Configure navigator
  NavigateAction.setNavigatorKey(NavigationService.navigatorKey);
  // Run app
  runApp(Bootstrap());
}

class Bootstrap extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return AsyncReduxProvider<AppState>.value(
        // Instead of StoreProvider.
        value: store!,
        child: App());
  }
}

class App extends StatefulWidget {
  @override
  _AppState createState() => _AppState();
}

class _AppState extends State<App> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),
      //platform: TargetPlatform.iOS
      title: 'bThermo',
      navigatorKey: NavigationService.navigatorKey,
      routes: routes,
      initialRoute: "/home",
    );
  }
}
