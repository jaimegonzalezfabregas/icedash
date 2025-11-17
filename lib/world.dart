import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/image_composition.dart';
import 'package:flame_camera_tools/flame_camera_tools.dart';
import 'package:flutter/material.dart';
import 'package:icedash/components/player.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/room_traversal.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/tile.dart';

import 'src/rust/api/main.dart';


class IceDashWorld extends World with HasGameReference {
  late CameraComponent camera;
  RoomTraversal roomTraversal = RoomTraversal();

  RoomComponent? _currentRoom;

  late Player player;

  @override
  Future<void> onLoad() async {
    player = Player(position: Vector2(100, 104));
    add(player);

    var destination = roomTraversal.getOnLoadDestination();

    await loadRoom(destination, Vector2(100, 100), Direction.north);
    player.push(Direction.north);
  }

  Future<RoomComponent> getRoom(Vector2 worldEntrancePosition, Direction stichDirection, GateDestination destination) async {
    var (board, entranceGateId) = await roomTraversal.getRoom(destination, stichDirection);

    return RoomComponent(worldEntrancePosition, stichDirection, board, entranceGateId);
  }

  Future<void> loadRoom(GateDestination destination, Vector2 worldStichPos, Direction stichDirection) async {
    double camTransitionDuration = 0.75;
    double camTransitionStaticPortion = 0.33;

    var transition = EffectController(duration: 0);

    if (_currentRoom != null) {
      transition = EffectController(
        curve: Curves.easeInOut,
        duration: camTransitionDuration,
        startDelay: camTransitionDuration * camTransitionStaticPortion,
      );
    }

    Vector2 dpos = stichDirection.dartVector();
    Vector2 worldEntrancePosition = worldStichPos + dpos;

    _currentRoom = await getRoom(worldEntrancePosition, stichDirection, destination);
    add(_currentRoom!);

    Rect newFocus = _currentRoom!.worldBB;

    queueZoomTransition(
      camTransitionDuration * (1 + camTransitionStaticPortion * 2),
      min(game.size.x / newFocus.width, game.size.y / newFocus.height),
    );

    camera.lookAt(newFocus.center.toVector2(), transition);

    player.remainingMovesReset = _currentRoom!.room.maxMovementCount;
    if (player.remainingMovesReset != null) {
      player.remainingMoves = player.remainingMovesReset!;
    } else {
      player.remainingMoves = null;
    }
    // player.push(stichDirection);

    player.rescueIfOutside(stichDirection);
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
  void onGameResize(Vector2 size) {
    Rect focus = _currentRoom!.worldBB;
    if (_currentRoom != null) {
      camera.zoomTo(min(game.size.x / focus.width, game.size.y / focus.height), LinearEffectController(0));
    }
    super.onGameResize(size);
  }

  void setCamera(CameraComponent cam) {
    camera = cam;
  }

  Future<bool> canWalkInto(Vector2 og, Vector2 dst, Direction dir, bool userPush, bool predicting) async {
    bool ret = await _currentRoom!.canWalkInto(og, dst, dir, userPush, predicting);
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
