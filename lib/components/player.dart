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
  int? remainingMoves;
  int? remainingMovesReset;

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
      position = game.idWorld.resetPlayerPos();
      push(game.idWorld.getResetDirection());
      remainingMoves = remainingMovesReset;
    }
  }

  Future<Vector2> predictHit(Direction dir, bool firstPush) async {
    Vector2 cursor = position;
    Vector2 delta = dir.dartVector();

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

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secondsToHit - secPerStep / 2),
        onComplete: () async {
          sprite = await Sprite.load('player_${axis}0001.png');
          animationState = "${axis}0001";
        },
      ),
    );

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(secondsToHit),
        onComplete: () async {
          sprite = await Sprite.load('player_idle.png');
          animationState = "idle";
        },
      ),
    );
  }

  Future<void> onHit(Direction dir, int movementLenght) async {
    sliding = false;

    Tile hitTile = await game.idWorld.getTile(position + dir.dartVector());

    bool consecuences = await game.idWorld.hit(position + dir.dartVector(), dir);

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
  }

  bool moving = false;

  void push(Direction dir, {bool firstPush = true}) async {
    if (moving) {
      return;
    }
    moving = true;

    Vector2 destination = await predictHit(dir, firstPush);

    int movementLenght = (destination - position).length.floor();

    MoveToEffect effect = MoveToEffect(
      destination,
      LinearEffectController(movementLenght * secPerStep),
      onComplete: () {
        onHit(dir, movementLenght);
        moving = false;
      },
    );

    add(effect);
  }

  void rescueIfOutside(Direction rescueDir) async {
    while (moving) {
      await Future.delayed(Duration.zero);
    }

    if ((await game.idWorld.getTile(position)) is Tile_Outside) {
      push(rescueDir, firstPush: false);
    }
  }
}
