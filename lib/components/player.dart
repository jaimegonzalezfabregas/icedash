import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:flutter/services.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/src/rust/api/tile.dart';

class Player extends SpriteComponent with HasGameReference<IceDashGame> {
  Player({super.position}) : super(priority: 20, size: Vector2.all(1), anchor: Anchor.center);

  double secPerStep = 0.07;
  bool sliding = false;
  Direction? buffered;
  int? remainingMoves;
  int? remainingMovesReset;
  int movementLenght = 0;

  String animationState = "idle";

  @override
  Future<void> onLoad() async {
    sprite = await Sprite.load('player_idle.png');

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

  void predictHit(Direction dir) async {
    Vector2 cursor = position;
    bool userPush = true;
    Vector2 delta =  dir.dartVector();

    while ((await game.idWorld.canWalkInto(cursor, cursor + delta, dir, userPush, true))) {
      userPush = false;
      cursor = cursor + delta;
    }

    game.idWorld.predictedHit(position, cursor + delta, dir);
  }

  void push(Direction dir, {bool userPush = true}) async {
    late String axis;

    switch (dir) {
      case Direction.north:
      case Direction.south:
        axis = "vertical";
        break;
      case Direction.east:
        axis = "right";
        break;
      case Direction.west:
        axis = "left";
        break;
    }

    if (animationState == "idle") {
      add(
        FunctionEffect(
          (_, __) {},
          LinearEffectController(secPerStep / 2),
          onComplete: () async {
            sprite = await Sprite.load('player_${axis}0001.png');
            animationState = "${axis}0001";
          },
        ),
      );

      add(
        FunctionEffect(
          (_, __) {},
          LinearEffectController(secPerStep),
          onComplete: () async {
            sprite = await Sprite.load('player_${axis}0002.png');
            animationState = "${axis}0002";
          },
        ),
      );
    }

    Vector2 delta = dir.dartVector();

    if (sliding) {
      if (userPush) {
        buffered = dir;
      }
      return;
    }

    sliding = true;

    if (userPush) {
      predictHit(dir);
    }

    if (!(await game.idWorld.canWalkInto(position, position + delta, dir, userPush, false))) {
      sliding = false;

      add(
        FunctionEffect(
          (_, __) {},
          LinearEffectController(secPerStep / 2),
          onComplete: () async {
            sprite = await Sprite.load('player_${axis}0001.png');
            animationState = "${axis}0001";
          },
        ),
      );

      add(
        FunctionEffect(
          (_, __) {},
          LinearEffectController(secPerStep),
          onComplete: () async {
            sprite = await Sprite.load('player_idle.png');
            animationState = "idle";
          },
        ),
      );

      Tile hitTile = await game.idWorld.getTile(position + delta);

      bool consecuences = await game.idWorld.hit(position + delta, dir);

      int moveI = remainingMoves == null ? 1 : remainingMoves!;

      String audio = 'move_${min(moveI, 17)}.mp3';

      if (movementLenght != 0 || consecuences) {
        if (remainingMoves != null) {
          remainingMoves = remainingMoves! - 1;
          print("remaining moves: $remainingMoves");
          if (remainingMoves == 0) {
            if (hitTile is! Tile_Outside) {
              audio = "too_many_moves.mp3";
              reset();
            }
          }
        }
      }

      if (hitTile is! Tile_Gate) {
         FlameAudio.play(audio);
      } else {
        if (hitTile.field0 is GateMetadata_EntryOnly) {
           FlameAudio.play(audio);
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

    EffectController ec = LinearEffectController(secPerStep);

    MoveByEffect effect = MoveByEffect(delta, ec);

    effect.onComplete = () {
      sliding = false;
      movementLenght += 1;
      push(dir, userPush: false);
    };

    add(effect);
  }

  void rescueIfOutside(Direction rescueDir) async {

    if ((await game.idWorld.getTile(position)) is Tile_Outside) {
      push(rescueDir, userPush: false);
    }
  }
}
