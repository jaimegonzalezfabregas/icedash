import 'dart:math';

import 'package:flame/camera.dart';
import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/events.dart';
import 'package:flame/game.dart';
import 'package:flame/image_composition.dart';
import 'package:flame_camera_tools/flame_camera_tools.dart';
import 'package:flutter/material.dart';
import 'package:icedash/components/player.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/room_traversal.dart';
import 'package:icedash/src/rust/api/main.dart';

import 'package:icedash/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Flame.device.fullScreen();
  await RustLib.init();

  runApp(GameWidget(game: IceDashGame()));
}

class IceDashGame extends FlameGame with HasKeyboardHandlerComponents {
  late IceDashWorld idWorld;

  IceDashGame() {
    idWorld = IceDashWorld();
    CameraComponent camera = CameraComponent(world: idWorld, viewport: MaxViewport());
    idWorld.setCamera(camera);

    super.world = idWorld;
    super.camera = camera;
  }
}

class IceDashWorld extends World {
  late CameraComponent camera;
  RoomTraversal roomTraversal = RoomTraversal();

  RoomComponent? _lastRoom;
  RoomComponent? _currentRoom;

  late Vector2 screenSize = Vector2.all(1);
  late Player player;

  @override
  Future<void> onLoad() async {
    player = Player(position: Vector2(0, 0));
    add(player);

    var board = roomTraversal.getOnLoadRoom();

    setCurrentRoom(board, Vector2(0, 0), Direction.north);
  }

  void nextRoom(Vector2 exitPos, Direction exitDirection) {
    Vector2 dpos = Vector2.array(exitDirection.dartVector());
    Vector2 entrancePostion = exitPos + dpos;

    var board = roomTraversal.getNextRoom(Pos(x: (exitPos.x).round(), y: (exitPos.y).round()));

    setCurrentRoom(board, entrancePostion, exitDirection);
  }

  void setCurrentRoom(Room room, Vector2 worldEntrancePosition, Direction stichDirection) {
    _lastRoom = _currentRoom;
    _currentRoom = RoomComponent(worldEntrancePosition, stichDirection, room);

    var transition = EffectController(duration: 0);

    const camTransitionDuration = 0.8;

    if (_lastRoom != null) {
      transition = EffectController(curve: Curves.easeInOut, duration: camTransitionDuration);
    }

    camera.lookAt(_currentRoom!.worldBB.center.toVector2(), transition);

    zoomTransition(camTransitionDuration, min(screenSize.x / _currentRoom!.worldBB.width, screenSize.y / _currentRoom!.worldBB.height));

    if (_lastRoom != null) {
      var fadingOutRoom = _lastRoom!;
      _lastRoom = null;
      fadingOutRoom.fadeOut((){
         remove(fadingOutRoom);
      });
      
    }
    add(_currentRoom!);
    player.push(stichDirection, force: true);

    player.resetPos = _currentRoom!.resetWorldPos;
    if (room is Room_Trial) {
      player.remainingMoves = room.getMaxMovementCount()!;
      player.remainingMovesReset = room.getMaxMovementCount()! - 1;
    } else {
      player.remainingMoves = null;
      player.remainingMovesReset = null;
    }
  }



  zoomTransition(double duration, double endValue) {
    var zoomOutEfect = CurvedEffectController(duration / 2, Curves.easeInOut);
    var zoomInEffect = CurvedEffectController(duration / 2, Curves.easeInOut);

    camera.viewfinder.add(
      ScaleEffect.to(
        Vector2.all(endValue * 0.8),
        zoomOutEfect,
        onComplete: () {
          camera.viewfinder.add(ScaleEffect.to(Vector2.all(endValue), zoomInEffect));
        },
      ),
    );
  }

  @override
  void onGameResize(Vector2 size) {
    screenSize = size;
    if (_currentRoom != null) {
      camera.zoomTo(min(screenSize.x / _currentRoom!.worldBB.width, screenSize.y / _currentRoom!.worldBB.height), LinearEffectController(0));
    }
    super.onGameResize(size);
  }

  void setCamera(CameraComponent cam) {
    camera = cam;
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    bool ret = _currentRoom!.canWalkInto(origin, dst);
    return ret;
  }

  void hit(Vector2 pos, Direction dir){
    _currentRoom!.hit(pos, dir);
  }

  Tile getTile(Vector2 position) {
    return _currentRoom!.getTile(position);
  }
}
