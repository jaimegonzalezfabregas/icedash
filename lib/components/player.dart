import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flutter/services.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';

class Player extends SpriteComponent with HasGameReference<IceDashGame> {
  Player({super.position}) : super(priority: 20, size: Vector2.all(1), anchor: Anchor.center);

  double timePerStep = 0.1;
  bool sliding = false;
  Direction? buffered;
  int? remainingMoves;
  int? remainingMovesReset;
  int movementLenght = 0;

  
  @override
  Future<void> onLoad() async {
    sprite = await Sprite.load('player.png');

    add(
      KeyboardListenerComponent(
        keyDown: {
          LogicalKeyboardKey.keyA: (keysPressed) {
            push(Direction.west);
            return true;
          },
          LogicalKeyboardKey.keyD: (keysPressed) {
            push(Direction.east);
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
    if (!sliding) {
      game.idWorld.reset();
      buffered = null;
      position = game.idWorld.resetPlayerPos();
      push(game.idWorld.getResetDirection(), userPush: true);
      remainingMoves = remainingMovesReset;
    }
  }

  void push(Direction dir, {bool userPush = true}) async{
    Vector2 delta = Vector2.array(dir.dartVector());

    if (sliding) {
      buffered = dir;
      return;
    }

    if (!(await game.idWorld.canWalkInto(position, position + delta, dir, userPush))) {
      bool consecuences = await game.idWorld.hit(position + delta, dir);

      if (movementLenght != 0 || consecuences) {
        if (remainingMoves != null) {
          remainingMoves = remainingMoves! - 1;
          print("remaining moves $remainingMoves");
          if (remainingMoves == 0) {
            reset();
          }
        }
      }

      movementLenght = 0;

      if (buffered != null) {
        Direction d = buffered!;
        buffered = null;
        push(d);
      }

      return;
    }

    sliding = true;

    EffectController ec = LinearEffectController(timePerStep);

    MoveByEffect effect = MoveByEffect(delta, ec);

    effect.onComplete = () {
      sliding = false;
      movementLenght += 1;
      push(dir, userPush: false);
    };

    add(effect);
  }
}
