import 'dart:async';
import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/components/sign.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/world.dart';

class Gate extends Actor with HasGameReference<IceDashGame> {
  RoomComponent room;
  int gateId;
  double secPerStep = 0.07;
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
    // UNREACHEABLE
    return false;
  }

  @override
  void predictedHit(Vector2 startingPos, Direction dir) {
    var secsToExit = secPerStep * ((startingPos - position).length);

    dartWorkerHalt(millis: BigInt.from(secsToExit * 1000 + 1));

    add(
      OpacityEffect.fadeOut(
        LinearEffectController(secsToExit),
        onComplete: () {
          room.fadeOut(gateId);
          removeFromParent();
        },
      ),
    );

    world.loadRoom(destination, position, dir);
  }
}
