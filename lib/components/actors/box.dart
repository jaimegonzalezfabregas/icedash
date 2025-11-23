import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/src/rust/api/direction.dart';

class Box extends Actor {
  double secPerStep = 0.07;

  RoomComponent room;
  Box(this.room, {super.position}) : super("box.png");

  @override
  Future<bool> hit(Direction dir) async {
    FlameAudio.play('hit_box.mp3');

    return push(dir);
  }

  Future<Vector2> predictHit(Direction dir) async {
    Vector2 cursor = position;
    Vector2 delta = dir.dartVector();

    while ((await room.canBoxWalkInto(cursor + delta, dir))) {
      cursor = cursor + delta;
    }

    return cursor;
  }

  Future<bool> push(Direction dir) async {
    Vector2 destination = await predictHit(dir);
    int movementLenght = (destination - position).length.floor();

    print("moving box from $position to $destination, in $movementLenght ");

    super.colision = false;
    add(
      MoveToEffect(
        destination,
        LinearEffectController(secPerStep * movementLenght),
        onComplete: () {
          super.colision = true;
          room.hit(destination + dir.dartVector(), dir, box: true);
        },
      ),
    );

    return true;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
