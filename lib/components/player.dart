import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flutter/services.dart';
import 'package:icedash/direction.dart';
import 'package:icedash/main.dart';
import 'package:icedash/tile.dart';

class Player extends SpriteComponent with HasGameReference<IceDashGame> {
  Player({super.position})
    : super(priority: 1, size: Vector2.all(100), anchor: Anchor.topLeft);

  double timePerStep = 0.1;
  bool sliding = false;
  Direction? buffered;

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
        },
      ),
    );
  }

  void push(Direction dir, {bool force = false}) {
    if (!force) {
      if (sliding) {
        buffered = dir;
        return;
      }

      if (!game.idWorld.canWalkInto(position, position + dir.vector * 100)) {
        if (buffered != null) {
          Direction d = buffered!;
          buffered = null;
          push(d);
        }
        return;
      }
    }

    Vector2 delta = dir.vector;

    sliding = true;

    EffectController ec = LinearEffectController(timePerStep);

    MoveByEffect effect = MoveByEffect(delta * 100, ec);

    effect.onComplete = () {
      sliding = false;
      var standingOn = game.idWorld.getTile(position);

      if (standingOn == Tile.gate) {
        game.idWorld.nextRoom(position, dir);
      }

      if (standingOn == Tile.gate ||
          standingOn == Tile.entrance ||
          standingOn == Tile.ice) {
        push(dir);
      }
    };

    add(effect);
  }
}
