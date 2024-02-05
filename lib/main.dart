import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/gallery.dart';
import 'package:repupil/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  int name = 0;

  @override
  void initState() {
    getGalleryIdsFromNozomi(language: "all", area: "popular", tag: "")
        .then((result) => {
              setState(() {
                print(result.last);
                name = result.first;
              })
            });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge test')),
        body: Center(
          child: Text('Action: Call Rust `greet("Snoxx")`\nResult: `$name`'),
        ),
      ),
    );
  }
}
