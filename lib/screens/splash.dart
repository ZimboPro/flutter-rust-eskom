import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:shared_preferences/shared_preferences.dart';

const apiPreferenceKey = "ESKOM_API_KEY";

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
    _prefs.then((value) {
      final apiKey = value.get(apiPreferenceKey);
      if (apiKey == null) {
        context.replaceNamed("setup");
      } else {
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
