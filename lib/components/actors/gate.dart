import 'dart:async';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';

class Gate extends Actor with HasGameReference<IceDashGame> {
  RoomComponent room;
  BigInt gateId;
  double timePerStep = 0.1;
  bool fadeIn;

  (String, BigInt) destination;

  Gate(this.room, this.gateId, this.destination, {required this.fadeIn, super.position, super.angle}) : super("fade.png", colision: false, selffade: fadeIn);

  @override
  FutureOr<void> onLoad() {
    if (fadeIn) {
      print("fading in gate");
      super.opacity = 0;
      add(OpacityEffect.fadeIn(EffectController(duration: 1, startDelay: 1), onComplete: () => print("done"),));
    }
    return super.onLoad();
  }

  @override
  bool hit(Direction dir) {
    print("hit gate");

    dartWorkerHalt(millis: BigInt.from(timePerStep * 1000 * 4));
    game.idWorld.goToRoom(destination, position, dir);

    add(
      OpacityEffect.fadeOut(
        LinearEffectController(timePerStep),
        onComplete: () {
          room.fadeOut(gateId);
          removeFromParent();
        },
      ),
    );

    return false;
  }
}
