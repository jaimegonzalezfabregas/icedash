import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'package:icedash/game.dart';

import 'package:icedash/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Flame.device.fullScreen();
  await RustLib.init();

  runApp(GameWidget(game: IceDashGame()));
}
