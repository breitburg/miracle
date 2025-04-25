import 'dart:io';

import 'package:miracle/app.dart';
import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  if (Platform.isMacOS) {
    const config = MacosWindowUtilsConfig(
      toolbarStyle: NSWindowToolbarStyle.unified,
    );

    await config.apply();
  }

  runApp(const MiracleApp());
}
