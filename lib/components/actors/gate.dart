import 'dart:async';
import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/palette.dart';
import 'package:flame/text.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';

class Gate extends Actor with HasGameReference<IceDashGame> {
  RoomComponent room;
  BigInt gateId;
  double timePerStep = 0.1;
  bool fadeIn;

  Direction inner_direction;
  (String, BigInt) destination;
  String? lable;

  Gate(this.room, this.gateId, this.destination, this.inner_direction, this.lable, {required this.fadeIn, super.position})
    : super(
        "fade.png",
        colision: false,
        selffade: fadeIn,
        angle: switch (inner_direction) {
          Direction.west => pi / 2,
          Direction.north => pi,
          Direction.east => -pi / 2,
          Direction.south => 0,
        },
      );

  @override
  FutureOr<void> onLoad() {
    TextStyle a = TextStyle(fontSize: 1.0, color: Color.fromARGB(255, 255, 255, 255), fontFamily: "BoldPixels");
    TextStyle b = TextStyle(fontSize: 1.0, color: Color.fromARGB(255, 58, 38, 145), fontFamily: "BoldPixels");
    final regular = TextPaint(style: a);
    final shadow = TextPaint(style: b);

    if (fadeIn) {
      super.opacity = 0;
      add(OpacityEffect.fadeIn(EffectController(duration: 1, startDelay: 1)));
    }

  var text = "SinglePlayer";

    add(
      TextComponent(
        position: Vector2(-regular.getLineMetrics(text).width / 2 + 0.5, -1 - 5 * (1 / 16)),
        text: text,
        size: Vector2(regular.getLineMetrics(text).width, 1),
        textRenderer: regular,
        priority: 100,
      ),
    );

    add(
      TextComponent(
        position: Vector2(-shadow.getLineMetrics(text).width / 2 + 0.5 - (1 / 16), -1 - 4*(1 / 16)),
        text: text,
        size: Vector2(shadow.getLineMetrics(text).width, 1),
        textRenderer: shadow,
        priority: 99,
      ),
    );

    return super.onLoad();
  }

  @override
  bool hit(Direction dir) {
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
