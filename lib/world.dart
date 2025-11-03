import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/image_composition.dart';
import 'package:flame_camera_tools/flame_camera_tools.dart';
import 'package:flutter/material.dart';
import 'package:icedash/components/player.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/room_traversal.dart';
import 'package:icedash/src/rust/logic/pos.dart';

import 'src/rust/api/main.dart';

class IceDashWorld extends World with HasGameReference{
  late CameraComponent camera;
  RoomTraversal roomTraversal = RoomTraversal();

  RoomComponent? _currentRoom;

  late Player player;

  @override
  Future<void> onLoad() async {
    player = Player(position: Vector2(0, 0));
    add(player);

    var destination = roomTraversal.getOnLoadDestination();

    goToRoom(destination, Vector2(0, 0), Direction.north);
    player.push(Direction.north);
  }

  void goToRoom((String, BigInt) destination, Vector2 worldStichPos, Direction exitDirection) {
    Vector2 dpos = Vector2.array(exitDirection.dartVector());
    Vector2 entrancePostion = worldStichPos + dpos;

    var board = roomTraversal.getRoom(destination.$1, Pos(x: (worldStichPos.x).round(), y: (worldStichPos.y).round()));

    setCurrentRoom(board, entrancePostion, exitDirection, destination.$2);
  }

  void setCurrentRoom(DartBoard room, Vector2 worldEntrancePosition, Direction stichDirection, BigInt entranceGateId) {
    var lastRoom = _currentRoom;
    _currentRoom = RoomComponent(worldEntrancePosition, stichDirection, room, entranceGateId);

    var transition = EffectController(duration: 0);

    const camTransitionDuration = 0.8;
    const camTransitionStaticPortion = 0.2;

    if (lastRoom != null) {
      transition = EffectController(
        curve: Curves.easeInOut,
        duration: camTransitionDuration,
        startDelay: camTransitionDuration * camTransitionStaticPortion,
      );
    }

    zoomTransition(
      camTransitionDuration * (1 + camTransitionStaticPortion * 2),
      min(game.size.x / _currentRoom!.worldBB.width, game.size.y / _currentRoom!.worldBB.height),
    );

    camera.lookAt(_currentRoom!.worldBB.center.toVector2(), transition);

    add(_currentRoom!);

    player.remainingMoves = room.getMaxMovementCount();
    player.remainingMovesReset = room.getMaxMovementCount();
  }

  double? lastZoomVal;

  zoomTransition(double duration, double endValue) {
    var lastVal = lastZoomVal ?? endValue;

    var middlePoint = min(endValue, lastVal) * 0.9;

    var zoomOutEfect = CurvedEffectController(duration / 2, Curves.easeInOut);
    var zoomInEffect = CurvedEffectController(duration / 2, Curves.easeInOut);

    camera.viewfinder.add(
      ScaleEffect.to(
        Vector2.all(middlePoint),
        zoomOutEfect,
        onComplete: () {
          camera.viewfinder.add(ScaleEffect.to(Vector2.all(endValue), zoomInEffect));
        },
      ),
    );

    lastZoomVal = endValue;
  }

  @override
  void onGameResize(Vector2 size) {
    if (_currentRoom != null) {
      camera.zoomTo(min(game.size.x / _currentRoom!.worldBB.width, game.size.y / _currentRoom!.worldBB.height), LinearEffectController(0));
    }
    super.onGameResize(size);
  }

  void setCamera(CameraComponent cam) {
    camera = cam;
  }

  bool canWalkInto(Vector2 og, Vector2 dst, Direction dir, bool userPush) {
    bool ret = _currentRoom!.canWalkInto(og, dst, dir, userPush);
    return ret;
  }

  bool hit(Vector2 pos, Direction dir) {
    return _currentRoom!.hit(pos, dir);
  }

  Tile getTile(Vector2 position) {
    return _currentRoom!.getTile(position);
  }

  reset() {
    _currentRoom!.reset();
  }

  Direction getResetDirection() {
    return _currentRoom!.entranceDirection;
  }

  Vector2 resetPlayerPos() {
    return _currentRoom!.entranceWorldPos;
  }
}
