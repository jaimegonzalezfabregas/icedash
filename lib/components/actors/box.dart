import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/src/rust/api/main.dart';

class Box extends Actor {
  double timePerStep = 0.07;

  RoomComponent room;
  Box(this.room, {super.position}) : super("box.png");

  @override
  Future<bool> hit(Direction dir) async {
    FlameAudio.play('hit_box.mp3');

    var delta = Vector2.array(dir.dartVector());
    if (!await room.canBoxWalkInto(position + delta, dir)) {
      await room.hit(position + delta, dir, box: true);
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

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
