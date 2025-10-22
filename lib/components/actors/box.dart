import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/src/rust/api/main.dart';

class Box extends Actor {
  double timePerStep = 0.1;

  RoomComponent room;
  Box(this.room, {super.position}) : super("box.png");

  @override
  bool hit(Direction dir) {
    if (!room.canBoxWalkInto(position + Vector2.array(dir.dartVector()), dir)) {
      return false;
    }

    super.colision = false;
    add(
      MoveByEffect(
        Vector2.array(dir.dartVector()),
        LinearEffectController(timePerStep),
        onComplete: () {
          super.colision = true;

          hit(dir);
        },
      ),
    );

    return true;
  }
}
