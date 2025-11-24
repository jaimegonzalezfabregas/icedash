import 'dart:ui';

import 'package:flame/camera.dart';
import 'package:flame/components.dart';
import 'package:flame/events.dart';
import 'package:flame/game.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/world.dart';

class IceDashGame extends FlameGame with HasKeyboardHandlerComponents, DragCallbacks, LongPressDetector {
  late IceDashWorld idWorld;

  @override
  Color backgroundColor() => const Color.fromARGB(255, 221, 238, 255);

  IceDashGame() {
    idWorld = IceDashWorld();
    CameraComponent camera = CameraComponent(world: idWorld, viewport: MaxViewport());
    idWorld.setCamera(camera);

    super.world = idWorld;
    super.camera = camera;
  }

  @override
  void onLongPress() {
    idWorld.player.reset();
  }

  Vector2? dragStart;
  Vector2? dragEnd;

  @override
  void onDragStart(DragStartEvent event) {
    dragStart = event.localPosition;
    dragEnd = event.localPosition;
    super.onDragStart(event);
  }

  @override
  void onDragUpdate(DragUpdateEvent event) {
    dragEnd = event.localEndPosition;
    super.onDragUpdate(event);
  }

  @override
  void onDragEnd(DragEndEvent event) {
    Vector2 dragVector = dragEnd! - dragStart!;

    if (dragVector.x.abs() > dragVector.y.abs()) {
      if (dragVector.x.sign > 0) {
        idWorld.player.push(Direction.east);
      } else {
        idWorld.player.push(Direction.west);
      }
    } else {
      if (dragVector.y.sign > 0) {
        idWorld.player.push(Direction.south);
      } else {
        idWorld.player.push(Direction.north);
      }
    }

    super.onDragEnd(event);
  }
}
