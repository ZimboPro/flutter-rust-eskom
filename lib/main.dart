import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:window_manager/window_manager.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';
import 'screens/area.dart';
import 'screens/setup.dart';
import 'screens/splash.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();

  const WindowOptions windowOptions = WindowOptions(
      size: Size(800, 600),
      minimumSize: Size(800, 600),
      center: true,
      backgroundColor: Colors.transparent,
      skipTaskbar: false,
      titleBarStyle: TitleBarStyle.normal);
  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
    await windowManager.focus();
  });

  runApp(const MyApp());
}

final _router = GoRouter(
  routes: [
    GoRoute(
      name: "home",
      path: '/home',
      builder: (context, state) {
        final key = state.queryParameters["apiKey"]!;
        return MyHomePage(apiKey: key);
      },
    ),
    GoRoute(
      name: "second",
      path: "/second",
      builder: (context, state) => const SecondPage(),
    ),
    GoRoute(
      path: "/",
      name: "splash",
      builder: (context, state) => const SplashScreen(),
    ),
    GoRoute(
      name: "setup",
      path: "/setup",
      builder: (context, state) => const SetupPage(),
    ),
    GoRoute(
      name: "area",
      path: "/area",
      builder: (context, state) => AreaPage(
        apiKey: state.queryParameters['apiKey']!,
      ),
    )
  ],
);

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'Flutter Demo',
      routerConfig: _router,
      // theme: ThemeData(
      //   // This is the theme of your application.
      //   //
      //   // Try running your application with "flutter run". You'll see the
      //   // application has a blue toolbar. Then, without quitting the app, try
      //   // changing the primarySwatch below to Colors.green and then invoke
      //   // "hot reload" (press "r" in the console where you ran "flutter run",
      //   // or simply save your changes to "hot reload" in a Flutter IDE).
      //   // Notice that the counter didn't reset back to zero; the application
      //   // is not restarted.
      //   primarySwatch: Colors.blue,
      // ),
      // home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.apiKey}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String apiKey;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> with WindowListener {
  // These futures belong to the state and are only initialized once,
  // in the initState method.
  late Future<Platform> platform;
  late Future<bool> isRelease;
  late Stream<int> ticks;

  @override
  void initState() {
    super.initState();
    windowManager.addListener(this);
    platform = api.platform();
    isRelease = api.rustReleaseMode();
    ticks = api.tick();
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  @override
  void onWindowFocus() {
    // Make sure to call once.
    setState(() {});
    // do something
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: const Text("Home"),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
        child: Column(
          // Column is also a layout widget. It takes a list of children and
          // arranges them vertically. By default, it sizes itself to fit its
          // children horizontally, and tries to be as tall as its parent.
          //
          // Invoke "debug painting" (press "p" in the console, choose the
          // "Toggle Debug Paint" action from the Flutter Inspector in Android
          // Studio, or the "Toggle Debug Paint" command in Visual Studio Code)
          // to see the wireframe for each widget.
          //
          // Column has various properties to control how it sizes itself and
          // how it positions its children. Here we use mainAxisAlignment to
          // center the children vertically; the main axis here is the vertical
          // axis because Columns are vertical (the cross axis would be
          // horizontal).
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text("You're running on"),
            // To render the results of a Future, a FutureBuilder is used which
            // turns a Future into an AsyncSnapshot, which can be used to
            // extract the error state, the loading state and the data if
            // available.
            //
            // Here, the generic type that the FutureBuilder manages is
            // explicitly named, because if omitted the snapshot will have the
            // type of AsyncSnapshot<Object?>.
            FutureBuilder<List<dynamic>>(
              // We await two unrelated futures here, so the type has to be
              // List<dynamic>.
              future: Future.wait([platform, isRelease]),
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headlineMedium;
                if (snap.error != null) {
                  // An error has been encountered, so give an appropriate response and
                  // pass the error details to an unobstructive tooltip.
                  debugPrint(snap.error.toString());
                  return Tooltip(
                    message: snap.error.toString(),
                    child: Text('Unknown OS', style: style),
                  );
                }

                // Guard return here, the data is not ready yet.
                final data = snap.data;
                if (data == null) return const CircularProgressIndicator();

                // Finally, retrieve the data expected in the same order provided
                // to the FutureBuilder.future.
                final Platform platform = data[0];
                final release = data[1] ? 'Release' : 'Debug';
                final text = const {
                      Platform.Android: 'Android',
                      Platform.Ios: 'iOS',
                      Platform.MacApple: 'MacOS with Apple Silicon',
                      Platform.MacIntel: 'MacOS',
                      Platform.Windows: 'Windows',
                      Platform.Unix: 'Unix',
                      Platform.Wasm: 'the Web',
                    }[platform] ??
                    'Unknown OS';
                return Text('$text ($release)', style: style);
              },
            ),
            StreamBuilder<int>(
                stream: ticks,
                builder: (context, snap) {
                  final style = Theme.of(context).textTheme.headlineMedium;
                  // Error
                  final error = snap.error;
                  if (error != null)
                    return Tooltip(
                        message: error.toString(),
                        child: Text('Error', style: style));
                  // Check data
                  final data = snap.data;
                  if (data != null)
                    return Text('$data second(s)', style: style);
                  // Loading
                  return const CircularProgressIndicator();
                }),
            ElevatedButton(
                onPressed: () => context.push("/second"), child: Text("Second"))
          ],
        ),
      ),
    );
  }
}

class SecondPage extends StatelessWidget {
  const SecondPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Second Page"),
      ),
      body: Center(
        child: ElevatedButton(
            child: Text("Back"), onPressed: () => context.pop(true)),
      ),
    );
  }
}
