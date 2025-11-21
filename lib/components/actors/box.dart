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

  Future<bool> push(Direction dir) async {
    var delta = dir.dartVector();
    if (!await room.canBoxWalkInto(position + delta, dir)) {
      await room.hit(position + delta, dir, box: true);
      return false;
    }

    super.colision = false;
    add(
      MoveByEffect(
        dir.dartVector(),
        LinearEffectController(secPerStep),
        onComplete: () {
          super.colision = true;

          push(dir);
        },
      ),
    );

    return true;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
