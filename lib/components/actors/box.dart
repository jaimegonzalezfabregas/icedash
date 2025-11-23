import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/flame.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/config.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/src/rust/api/direction.dart';

class Box extends Actor {

  RoomComponent room;
  late SpriteAnimationComponent boxDisplay;
  Box(this.room, {super.position}) : super(null);

  @override
  void onLoad() async {
    boxDisplay = SpriteAnimationComponent.fromFrameData(
      await Flame.images.load('box.png'),
      size: Vector2.all(1),
      playing: false,
      SpriteAnimationData.sequenced(textureSize: Vector2(16, 16), amount: 16, stepTime: secPerStep/3),
    );
    add(boxDisplay);
  }

  @override
  Future<bool> hit(Direction dir) async {
    FlameAudio.play('hit_box.mp3');

    Vector2 destination = await predictHit(dir);
    int movementLenght = (destination - position).length.floor();

    if (movementLenght != 0) {
      boxDisplay.position = position - destination;

      position = destination;
      boxDisplay.playing = true;

      boxDisplay.add(
        MoveToEffect(
          Vector2.all(0),
          LinearEffectController(secPerStep * movementLenght),
          onComplete: () {
            boxDisplay.playing = false;

            room.hit(destination + dir.dartVector(), dir, box: true); // TODO not working
          },
        ),
      );

      return true;
    }
    return false;
  }

  Future<Vector2> predictHit(Direction dir) async {
    Vector2 cursor = position;
    Vector2 delta = dir.dartVector();

    while ((await room.canBoxWalkInto(cursor + delta, dir))) {
      cursor = cursor + delta;
    }

    return cursor;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
