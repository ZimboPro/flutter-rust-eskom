import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../ffi.dart';

const apiPreferenceKey = "ESKOM_API_KEY";
const hasAreaKey = "LOCATION_SELECTED";

class SplashScreen extends StatefulWidget {
  const SplashScreen({super.key});

  @override
  State<SplashScreen> createState() => _SplashScreenState();
}

class _SplashScreenState extends State<SplashScreen> {
  final Future<SharedPreferences> _prefs = SharedPreferences.getInstance();

  @override
  void initState() {
    super.initState();
    _prefs.then((value) async {
      final apiKey = value.getString(apiPreferenceKey);
      final hasLocation = value.getBool(hasAreaKey);
      final supportDir = await getApplicationSupportDirectory();
      api.setCacheDir(cacheDir: supportDir.path);
      if (apiKey == null) {
        context.replaceNamed("setup");
      } else if (apiKey.isNotEmpty &&
          (hasLocation == null || hasLocation == false)) {
        await api.setApiKey(apiKey: apiKey);
        context.replaceNamed("area", queryParameters: {"apiKey": apiKey});
      } else {
        await api.setApiKey(apiKey: apiKey);
        context.replaceNamed("home", queryParameters: {"apiKey": apiKey});
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Column(
            children: [Text("Loading details"), CircularProgressIndicator()]),
      ),
    );
  }
}
