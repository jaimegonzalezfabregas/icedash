import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/src/rust/api/direction.dart';

class BoxDisplay extends SpriteComponent {
  @override
  onLoad() async {
    super.sprite = await Sprite.load("box.png");
    super.size = Vector2.all(1);
    super.bleed = 0.01;
  }
}

class Box extends Actor {
  double secPerStep = 0.07;

  RoomComponent room;
  late BoxDisplay boxDisplay;
  Box(this.room, {super.position}) : super("box.png");

  @override
  void onLoad() {
    boxDisplay = BoxDisplay();
    add(boxDisplay);
  }

  @override
  Future<bool> hit(Direction dir) async {
    FlameAudio.play('hit_box.mp3');

    Vector2 destination = await predictHit(dir);
    int movementLenght = (destination - position).length.floor();

    if (movementLenght != 0) {
      boxDisplay.position = position-destination;

      position = destination;

      boxDisplay.add(
        MoveToEffect(
          Vector2.all(0),
          LinearEffectController(secPerStep * movementLenght),
          onComplete: () {
            room.hit(destination + dir.dartVector(), dir, box: true);
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
