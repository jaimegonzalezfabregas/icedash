import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flutter/services.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';

class Player extends SpriteComponent with HasGameReference<IceDashGame> {
  Player({super.position}) : super(priority: 1, size: Vector2.all(1), anchor: Anchor.topLeft);

  double timePerStep = 0.1;
  bool sliding = false;
  Direction? buffered;
  Vector2? resetPos;

  @override
  Future<void> onLoad() async {
    sprite = await Sprite.load('player.png');

    add(
      KeyboardListenerComponent(
        keyDown: {
          LogicalKeyboardKey.keyA: (keysPressed) {
            push(Direction.east);
            return true;
          },
          LogicalKeyboardKey.keyD: (keysPressed) {
            push(Direction.west);
            return true;
          },
          LogicalKeyboardKey.keyW: (keysPressed) {
            push(Direction.north);
            return true;
          },
          LogicalKeyboardKey.keyS: (keysPressed) {
            push(Direction.south);
            return true;
          },
          LogicalKeyboardKey.keyR: (keysPressed) {
            reset();
            return true;
          },
        },
      ),
    );
  }

  void reset() {
    if (!sliding && resetPos != null) {
      position = resetPos!;
    }
  }

  void push(Direction dir, {bool force = false}) {
    Vector2 delta = Vector2.array(dir.dartVector());

    if (!force) {
      if (sliding) {
        buffered = dir;
        return;
      }

      if (!game.idWorld.canWalkInto(position, position + delta)) {
        if (buffered != null) {
          Direction d = buffered!;
          buffered = null;
          push(d);
        }
        return;
      }
    }


    sliding = true;

    EffectController ec = LinearEffectController(timePerStep);

    MoveByEffect effect = MoveByEffect(delta, ec);

    effect.onComplete = () {
      sliding = false;
      var standingOn = game.idWorld.getTile(position);

      if (standingOn is Tile_Gate) {
        game.idWorld.nextRoom(position, dir);
      }

      if (standingOn is Tile_Gate || standingOn is Tile_Entrance || standingOn is Tile_Ice) {
        push(dir);
      }
    };

    add(effect);
  }
}
