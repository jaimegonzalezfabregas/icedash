import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/image_composition.dart';
import 'package:flame_camera_tools/flame_camera_tools.dart';
import 'package:flutter/material.dart';
import 'package:icedash/components/player.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/room_traversal.dart';

import 'src/rust/api/main.dart';

class IceDashWorld extends World with HasGameReference {
  late CameraComponent camera;
  RoomTraversal roomTraversal = RoomTraversal();

  RoomComponent? _currentRoom;

  late Player player;

  @override
  Future<void> onLoad() async {
    player = Player(position: Vector2(0, 4));
    add(player);

    var destination = roomTraversal.getOnLoadDestination();

    await goToRoom(destination, Vector2(0, 0), Direction.north);
  }

  void predictedGoToRoom(GateDestination destination, Vector2 position, Direction dir) {}

  Future<void> goToRoom(GateDestination destination, Vector2 worldStichPos, Direction exitDirection) async {
    Vector2 dpos = Vector2.array(exitDirection.dartVector());
    Vector2 entrancePostion = worldStichPos + dpos;

    var board = await roomTraversal.getRoom(destination);

    await setCurrentRoom(board, entrancePostion, exitDirection, await destination.getGateId());
  }

  Future<void> setCurrentRoom(DartBoard room, Vector2 worldEntrancePosition, Direction stichDirection, BigInt entranceGateId) async {
    var lastRoom = _currentRoom;
    _currentRoom = RoomComponent(worldEntrancePosition, stichDirection, room, entranceGateId);

    var transition = EffectController(duration: 0);

    double camTransitionDuration = 0.75;
    double camTransitionStaticPortion = 0.33;

    if (lastRoom != null) {
      transition = EffectController(
        curve: Curves.easeInOut,
        duration: camTransitionDuration,
        startDelay: camTransitionDuration * camTransitionStaticPortion,
      );
    }

    Rect newFocus = await _currentRoom!.getWorldBB();

    queueZoomTransition(
      camTransitionDuration * (1 + camTransitionStaticPortion * 2),
      min(game.size.x / newFocus.width, game.size.y / newFocus.height),
    );

    camera.lookAt(newFocus.center.toVector2(), transition);

    add(_currentRoom!);

    player.remainingMovesReset = await room.getMaxMovementCount();
    if (player.remainingMovesReset != null) {
      player.remainingMoves = player.remainingMovesReset! + 1;
    } else {
      player.remainingMoves = null;
    }
    player.push(stichDirection);
  }

  double? lastZoomVal;
  List<(double, double)> zoomTransitionQueue = [];


  void queueZoomTransition(double duration, double endValue) {
    var lastVal = lastZoomVal ?? endValue;

    var middlePoint = min(endValue, lastVal) * 0.9;

    zoomTransitionQueue = [];

    zoomTransitionQueue.add((middlePoint, duration / 2));
    zoomTransitionQueue.add((endValue, duration / 2));

    lastZoomVal = endValue;

    doZoomTransition();
  }

  var zooming = false;

  void doZoomTransition() {
    if (!zooming) {
      if (zoomTransitionQueue.isNotEmpty) {
        (double, double) nextTransition = zoomTransitionQueue.removeAt(0);
        zooming = true;

        camera.viewfinder.add(ScaleEffect.to(Vector2.all(nextTransition.$1), CurvedEffectController(nextTransition.$2, Curves.linear)));

        add(
          FunctionEffect(
            (_, __) {},
            onComplete: () {
              zooming = false;
              doZoomTransition();
            },
            LinearEffectController(nextTransition.$2),
          ),
        );
      }
    }
  }

  @override
  void onGameResize(Vector2 size) async {
    Rect focus = await _currentRoom!.getWorldBB();
    if (_currentRoom != null) {
      camera.zoomTo(min(game.size.x / focus.width, game.size.y / focus.height), LinearEffectController(0));
    }
    super.onGameResize(size);
  }

  void setCamera(CameraComponent cam) {
    camera = cam;
  }

  Future<bool> canWalkInto(Vector2 og, Vector2 dst, Direction dir, bool userPush) async {
    bool ret = await _currentRoom!.canWalkInto(og, dst, dir, userPush);
    return ret;
  }

  Future<bool> hit(Vector2 pos, Direction dir) async {
    return _currentRoom!.hit(pos, dir);
  }

  Future<Tile> getTile(Vector2 position) async {
    return await (_currentRoom!.getTile(position));
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

  void predictedHit(Vector2 startingPos, Vector2 hitPos, Direction dir) {
    return _currentRoom!.predictedHit(startingPos, hitPos, dir);
  }
}
