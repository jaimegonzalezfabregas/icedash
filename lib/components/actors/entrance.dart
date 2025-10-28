import 'dart:async';

import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/main.dart';

class EntranceTmpIcePatch extends Actor {

  EntranceTmpIcePatch({super.position}) : super("ice.png", selffade: true);

  @override
  FutureOr<void> onLoad() async {
    super.opacity = 1;
    add(OpacityEffect.fadeOut(EffectController(duration: 1, startDelay: 1)));
    return super.onLoad();
  }

  @override
  bool hit(Direction dir) {
    return false;
  }
}
