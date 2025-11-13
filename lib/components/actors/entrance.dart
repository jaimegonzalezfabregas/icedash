import 'dart:async';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/direction.dart';

class EntranceTmpIcePatch extends Actor {
  EntranceTmpIcePatch({super.position}) : super("ice.png", selffade: true);

  @override
  FutureOr<void> onLoad() async {
    opacity = 1;
    add(OpacityEffect.fadeOut(EffectController(duration: 1, startDelay: 1)));
    return super.onLoad();
  }

  @override
  Future<bool> hit(Direction dir) async {
    return false;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {
  }
}
