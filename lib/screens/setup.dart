import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_template/screens/splash.dart';
import 'package:go_router/go_router.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:url_launcher/url_launcher_string.dart';
import '../ffi.dart';

class SetupPage extends StatefulWidget {
  const SetupPage({super.key});

  @override
  State<SetupPage> createState() => _SetupPageState();
}

class _SetupPageState extends State<SetupPage> {
  final apiKeyController = TextEditingController();
  final Future<SharedPreferences> _prefs = SharedPreferences.getInstance();
  bool isValidApiKey = false;
  bool showErrorMessage = false;
  bool enableTestBtn = false;

  @override
  void dispose() {
    apiKeyController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text("Enter your Eskom-se-Push API key"),
            SizedBox(
              width: 400,
              child: TextField(
                autofocus: true,
                textAlign: TextAlign.center,
                controller: apiKeyController,
                onChanged: (value) {
                  setState(() {
                    enableTestBtn = value.trim().isNotEmpty;
                  });
                },
              ),
            ),
            Padding(
              padding: EdgeInsets.only(top: 10),
              child: ElevatedButton(
                onPressed: enableTestBtn
                    ? () async {
                        setState(() {
                          showErrorMessage = false;
                        });
                        ScaffoldMessenger.of(context).showSnackBar(
                            const SnackBar(content: Text("Testing API key")));
                        final valid = await api.testApiKey(
                            api: apiKeyController.value.text.trim());
                        if (valid) {
                          _prefs.then((value) => value.setString(
                              apiPreferenceKey,
                              apiKeyController.value.text.trim()));
                          setState(() {
                            isValidApiKey = true;
                            showErrorMessage = false;
                          });
                        } else {
                          setState(() {
                            isValidApiKey = false;
                            showErrorMessage = true;
                          });
                        }
                      }
                    : null,
                child: const Text("Test API Key"),
              ),
            ),
            RichText(
                text: TextSpan(children: [
              const TextSpan(
                  style: const TextStyle(color: Colors.black),
                  text: "If you don't have an API key, you can apply for it "),
              TextSpan(
                  text: "here",
                  style: const TextStyle(color: Colors.blue),
                  recognizer: TapGestureRecognizer()
                    ..onTap = () {
                      launchUrlString("https://eskomsepush.gumroad.com/l/api");
                    })
            ])),
            ElevatedButton(
                onPressed:
                    isValidApiKey ? () => context.replaceNamed("home") : null,
                child: const Text("Next")),
            shouldShowErrorMsg(),
            // ? const Text(
            //     "Your API key is invalid",
            //     style: TextStyle(color: Colors.red),
            //   )
            // : null,
          ],
        ),
      ),
    );
  }

  Widget shouldShowErrorMsg() {
    if (showErrorMessage) {
      return const Text(
        "Your API key is invalid",
        style: TextStyle(color: Colors.red),
      );
    } else {
      return Container();
    }
  }
}
