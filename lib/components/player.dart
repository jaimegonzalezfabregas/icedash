import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame_audio/flame_audio.dart';
import 'package:flutter/services.dart';
import 'package:icedash/config.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/game.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/tile.dart';


class Player extends SpriteComponent with HasGameReference<IceDashGame> {
  Player({super.position}) : super(priority: 20, size: Vector2.all(1), anchor: Anchor.center);

  List<Direction> movementQueue = [];

  int? remainingMoves;
  int? remainingMovesReset;

  String animationState = "idle";

  Random rnd = Random();
  Vector2 randomVector2() => (Vector2.random(rnd)- Vector2.random(rnd)) * 2;

  @override
  Future<void> onLoad() async {
    sprite = await Sprite.load('player/player_idle.png');

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

  bool moving = false;

  void reset() {
    if (moving) {
      return;
    }
    movementQueue = [];
    game.idWorld.reset();
    position = game.idWorld.resetPlayerPos();
    push(game.idWorld.getResetDirection());
    remainingMoves = remainingMovesReset;
  }

  Future<Vector2> predictHit(Direction dir) async {
    Vector2 cursor = position;
    Vector2 delta = dir.dartVector();
    bool firstPush = true;

    while ((await game.idWorld.canMove(cursor, cursor + delta, dir, firstPush))) {
      firstPush = false;
      cursor = cursor + delta;
    }

    game.idWorld.predictedHit(position, cursor + delta, dir);

    return cursor;
  }

  void animate(Direction dir, double secondsToHit) {
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

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secPerStep / 2),
        onComplete: () async {
          sprite = await Sprite.load('player/player_${axis}0001.png');
          animationState = "${axis}0001";
        },
      ),
    );

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secPerStep),
        onComplete: () async {
          sprite = await Sprite.load('player/player_${axis}0002.png');
          animationState = "${axis}0002";
        },
      ),
    );

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secondsToHit - secPerStep / 2),
        onComplete: () async {
          sprite = await Sprite.load('player/player_${axis}0001.png');
          animationState = "${axis}0001";
        },
      ),
    );

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secondsToHit),
        onComplete: () async {
          sprite = await Sprite.load('player/player_idle.png');
          animationState = "idle";
        },
      ),
    );
  }

  Future<void> movementDispatch() async {
    if (movementQueue.isEmpty) {
      return;
    }

    if (moving) {
      return;
    }

    moving = true;

    Direction dir = movementQueue.first;
    movementQueue.removeAt(0);

    Vector2 destination = await predictHit(dir);

    int movementLenght = (destination - position).length.floor();

    MoveToEffect effect = MoveToEffect(
      destination,
      LinearEffectController(movementLenght * secPerStep),
      onComplete: () async {
        moving = false;

        Tile hitTile = await game.idWorld.getTile(position + dir.dartVector());

        bool consecuences = await game.idWorld.hit(position + dir.dartVector(), dir);
        bool exiting = hitTile is Tile_Outside;

        int moveI = remainingMoves == null ? 1 : remainingMoves!;

        String audio = 'move_${min(moveI, 17)}.mp3';

        if ((movementLenght != 0 || consecuences) && !exiting) {
          if (remainingMoves != null) {
            remainingMoves = remainingMoves! - 1;
            // print("remaining moves: $remainingMoves");
            if (remainingMoves == 0) {
              if (hitTile is! Tile_Outside) {
                audio = "too_many_moves.mp3";
                reset();
              }
            }
          }

          FlameAudio.play(audio);
        }
        movementDispatch();
      },
    );

    add(effect);
  }

  void push(Direction dir) async {
    movementQueue.add(dir);
    movementDispatch();
  }

  void rescueIfOutside(Direction rescueDir) async {
    while (moving) {
      await Future.delayed(Duration.zero);
    }

    if ((await game.idWorld.getTile(position)) is Tile_Outside) {
      push(rescueDir);
    }
  }
}
