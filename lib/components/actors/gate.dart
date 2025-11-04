import 'dart:async';
import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/components/sign.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/world.dart';

class Gate extends Actor with HasGameReference<IceDashGame> {
  RoomComponent room;
  BigInt gateId;
  double timePerStep = 0.05;
  late IceDashWorld world;

  Direction innerDirection;
  GateDestination destination;
  String? lable;

  Gate(this.room, this.gateId, this.destination, this.innerDirection, this.lable, {super.position})
    : super(
        "fade.png",
        colision: false,
        angle: switch (innerDirection) {
          Direction.west => pi / 2,
          Direction.north => pi,
          Direction.east => -pi / 2,
          Direction.south => 0,
        },
      );

  @override
  Future<void> onLoad() async {
    if (lable != null) {
      add(Sign(lable!, -angle));
    }
    world = game.idWorld;

    return await super.onLoad();
  }

  @override
  Future<bool> hit(Direction dir) async {
    world.goToRoom(destination, position, dir);

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

  @override
  void predictedHit(Vector2 startingPos, Direction dir) {
    dartWorkerHalt(millis: BigInt.from(timePerStep * 1000 * ((startingPos - position).length + 1)));

    world.predictedGoToRoom(destination, position, dir);
  }
}
