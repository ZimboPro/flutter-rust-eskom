import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../ffi.dart';

class AreaPage extends StatefulWidget {
  const AreaPage({super.key, required this.apiKey});

  final String apiKey;

  @override
  State<AreaPage> createState() => _AreaPageState();
}

class _AreaPageState extends State<AreaPage> {
  final areaController = TextEditingController();

  bool hasArea = false;
  bool testingArea = false;

  List<AreaSearchResult> searchResults = [];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Expanded(
                  child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Text("Enter an area that should be monitored."),
                  // TODO improve wording
                  const Text("Be as accurate as possible before searching."),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      SizedBox(
                        width: 400,
                        child: TextField(
                          enabled: !testingArea,
                          autofocus: true,
                          textAlign: TextAlign.center,
                          controller: areaController,
                          // onChanged: (value) {
                          //   setState(() {
                          //     enableTestBtn = value.trim().isNotEmpty;
                          //   });
                          // },
                        ),
                      ),
                      Container(
                        padding: const EdgeInsets.all(10),
                        child: ElevatedButton(
                          child: const Text("Search"),
                          onPressed: () async {
                            setState(() {
                              testingArea = true;
                            });
                            final response = await api.areaSearch(
                                api: widget.apiKey,
                                searchTerm: areaController.text.trim());
                            setState(() {
                              searchResults = response;
                              testingArea = true;
                            });
                          },
                        ),
                      )
                    ],
                  ),
                  // searchResults.isNotEmpty
                  //     ? OverflowBox(
                  //         child: ListView.builder(
                  //             scrollDirection: Axis.vertical,
                  //             itemCount: searchResults.length,

                  //             itemBuilder: (context, index) {
                  //               final item = searchResults[index];
                  //               return ListTile(
                  //                 title: Text(
                  //                   item.name,
                  //                   style: const TextStyle(color: Colors.black),
                  //                 ),
                  //                 subtitle: Text(
                  //                   item.region,
                  //                   style: const TextStyle(color: Colors.black),
                  //                 ),
                  //               );
                  //             }),
                  //       )
                  //     : Container()
                ],
              )),
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Padding(
                    padding: const EdgeInsets.all(50),
                    child: ElevatedButton(
                        onPressed:
                            hasArea ? () => context.replaceNamed("home") : null,
                        child: const Text("Done")),
                  )
                ],
              )
            ]),
      ),
    );
  }
}
