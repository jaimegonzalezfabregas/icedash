import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'package:icedash/game.dart';

import 'package:icedash/src/rust/frb_generated.dart';
import 'package:just_audio/just_audio.dart';
// import 'package:audioplayers/audioplayers.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Flame.device.fullScreen();
  await RustLib.init();

  runApp(GameWidget(game: IceDashGame()));
}

Map<String, AudioPlayer> audioPlayerCache = {};

void playAudio(String assetPath) async {
  if (audioPlayerCache[assetPath] != null) {
    audioPlayerCache[assetPath]!.seek(Duration(seconds: 0));
    audioPlayerCache[assetPath]!.play();
  } else {
    AudioPlayer player = AudioPlayer();
    player.setAsset("assets/audio/$assetPath");
    player.play();
    audioPlayerCache[assetPath] = player;
  }

  // player.resume();
}
