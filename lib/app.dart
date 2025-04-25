import 'package:miracle/screens/home_screen.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:macos_ui/macos_ui.dart';

final router = GoRouter(
  routes: [
    GoRoute(
      path: HomeScreen.routeName,
      builder: (context, state) => const HomeScreen(),
    ),
  ],
);

class MiracleApp extends StatelessWidget {
  const MiracleApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MacosApp.router(
      title: 'Miracle',
      routerConfig: router,
      debugShowCheckedModeBanner: false,
    );
  }
}
