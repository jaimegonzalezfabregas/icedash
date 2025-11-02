import 'dart:async';
import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/palette.dart';
import 'package:flame/text.dart';
import 'package:flutter/material.dart';
import 'package:flutter/painting.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';

final big_num = 1000.0;

class MyTextBox extends TextBoxComponent {
  Color color;

  MyTextBox(String text, {required Vector2 size, required this.color, required Vector2 delta, required position, super.angle})
    : super(
        position: position + delta / 16 / 4,
        scale: Vector2.all(1 / big_num),
        size: size * big_num,
        text: text,
        textRenderer: TextPaint(
          style: TextStyle(fontSize: big_num / 4, color: color, fontFamily: "BoldPixels"),
        ),
        boxConfig: TextBoxConfig(maxWidth: 3, margins: EdgeInsets.all(0)),
        align: Anchor.center,
        anchor: Anchor.center,
      );
}

class Gate extends Actor with HasGameReference<IceDashGame> {
  RoomComponent room;
  BigInt gateId;
  double timePerStep = 0.1;

  Direction inner_direction;
  (String, BigInt) destination;
  String? lable;

  Gate(this.room, this.gateId, this.destination, this.inner_direction, this.lable, {super.position})
    : super(
        "fade.png",
        colision: false,
        angle: switch (inner_direction) {
          Direction.west => pi / 2,
          Direction.north => pi,
          Direction.east => -pi / 2,
          Direction.south => 0,
        },
      );

  @override
  FutureOr<void> onLoad() {
    if (lable != null) {
      for (var x in [
        (Color.fromARGB(255, 0, 0, 0), Vector2(0, -1)),
        (Color.fromARGB(255, 0, 0, 0), Vector2(1, 0)),
        (Color.fromARGB(255, 0, 0, 0), Vector2(0, 1)),
        (Color.fromARGB(255, 0, 0, 0), Vector2(-1, 0)),
        (Color.fromARGB(255, 255, 255, 255), Vector2(0, 0)),
      ]) {
        var t = (MyTextBox(lable!, delta: x.$2, color: x.$1, size: Vector2(1, 1), position: Vector2(0.5, 0.5), angle: -super.angle));
        add(t);
      }
    }

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
