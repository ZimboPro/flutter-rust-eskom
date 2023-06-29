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
  bool enableBtn = false;

  List<AreaSearchResult> searchResults = [];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
          child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisSize: MainAxisSize.max,
          children: [
            SizedBox(
              width: 300,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Text("Enter an area that should be monitored."),
                  // TODO improve wording
                  const Text("Be as accurate as possible before searching."),
                  SizedBox(
                    width: 300,
                    child: TextField(
                      enabled: !testingArea,
                      autofocus: true,
                      textAlign: TextAlign.center,
                      controller: areaController,
                      onChanged: (value) {
                        setState(() {
                          enableBtn = value.trim().isNotEmpty;
                        });
                      },
                    ),
                  ),
                  Container(
                    padding: const EdgeInsets.all(10),
                    child: ElevatedButton(
                      onPressed: enableBtn
                          ? () async {
                              setState(() {
                                testingArea = true;
                              });
                              final response = await api.areaSearch(
                                  searchTerm: areaController.text.trim());
                              setState(() {
                                searchResults = response;
                                testingArea = false;
                              });
                            }
                          : null,
                      child: const Text("Search"),
                    ),
                  )
                ],
              ),
            ),
            SizedBox(
              width: 300,
              child: Scrollbar(
                thumbVisibility: true,
                child: ListView.separated(
                    clipBehavior: Clip.antiAlias,
                    scrollDirection: Axis.vertical,
                    itemCount: searchResults.length,
                    separatorBuilder: (context, index) {
                      return const Divider();
                    },
                    itemBuilder: (context, index) {
                      final item = searchResults[index];
                      return ListTile(
                        title: Text(
                          "${item.name} - ${item.region}",
                          style: const TextStyle(color: Colors.black),
                        ),
                        onTap: () async {
                          ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                              content: Text(
                                  "Added '${item.name} - ${item.region}' to your list of areas.")));
                          await api.addArea(
                              apiKey: widget.apiKey, areaId: item.areaId);
                        },
                      );
                    }),
              ),
            )
          ],
        ),
      )),
      bottomNavigationBar: Row(
        mainAxisAlignment: MainAxisAlignment.end,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Padding(
            padding: const EdgeInsets.all(20),
            child: ElevatedButton(
                onPressed: hasArea
                    ? () => context.replaceNamed("home",
                        queryParameters: {'apiKey': widget.apiKey})
                    : null,
                child: const Text("Done")),
          )
        ],
      ),
    );
  }
}
