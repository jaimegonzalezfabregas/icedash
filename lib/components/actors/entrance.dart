import 'dart:async';

import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/main.dart';

class Entrance extends Actor {
  Entrance(super.asset, {super.position});

  @override
  FutureOr<void> onLoad() async {
    super.opacity = 0;
    add(OpacityEffect.fadeIn(EffectController(duration: 1, startDelay: 1)));
    return super.onLoad();
  }

  @override
  bool hit(Direction dir) {
    return false;
  }
}
