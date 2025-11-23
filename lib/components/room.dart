import 'dart:async';
import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/actors/box.dart';
import 'package:icedash/components/actors/entrance.dart';
import 'package:icedash/components/actors/gate.dart';
import 'package:icedash/components/actors/weak_wall.dart';
import 'package:icedash/components/sign.dart';
import 'package:icedash/extensions.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/src/rust/api/pos.dart';
import 'package:icedash/src/rust/api/tile.dart';

class RoomComponent extends Component with HasGameReference<IceDashGame> {
  DartBoard room;

  late Direction exitDirection;

  Vector2 entranceWorldPos;
  late Vector2 entranceRoomPos;
  Map<Pos, SpriteComponent> tileSpriteGrid = {};
  List<Actor> actorList = [];
  Direction entranceDirection;

  int entranceGateId;

  Vector2 mapPos2WorldVector(Pos p) {
    return p.dartVector() - entranceRoomPos + entranceWorldPos;
  }

  Future<Pos> worldVector2MapPos(Vector2 v) async {
    int x = (v.x - entranceWorldPos.x + (entranceRoomPos).x).round();
    int y = (v.y - entranceWorldPos.y + (entranceRoomPos).y).round();
    return Pos(x: x, y: y);
  }

  void reset() {
    buildSpriteGrid(1);
  }

  late Rect worldBB;

  RoomComponent(this.entranceWorldPos, this.entranceDirection, this.room, this.entranceGateId) {
    entranceRoomPos = room.gatePositions[entranceGateId].dartVector();

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - (entranceRoomPos).x - 0.5,
      entranceWorldPos.y - (entranceRoomPos).y - 0.5,
      (room.width).toDouble(),
      (room.height).toDouble(),
    );
  }

  Future fadeIn() async {
    var fadeDuration = 0.5;
    var rippleDuration = 0.1;
    double maxDelay = 0;

    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - entranceWorldPos).length * rippleDuration;
      sprite.opacity = 0;

      maxDelay = max(maxDelay, fadeDuration + d);
    }

    for (var sprite in actorList) {
      double d = (sprite.position - entranceWorldPos).length * rippleDuration;
      sprite.opacity = 0;

      maxDelay = max(maxDelay, fadeDuration + d);
    }

    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - entranceWorldPos).length * rippleDuration;

      await sprite.add(OpacityEffect.fadeIn(EffectController(duration: fadeDuration, startDelay: d)));
    }

    for (var actor in actorList) {
      if (!actor.selffade) {
        double d = (actor.position - entranceWorldPos).length * rippleDuration;

        await actor.add(OpacityEffect.fadeIn(EffectController(duration: fadeDuration, startDelay: d)));
      }
    }

    add(FunctionEffect((_, __) {}, EffectController(duration: maxDelay + fadeDuration + 1)));
  }

  void fadeOut(int exitGateId) {
    Vector2 exitWorldPos = mapPos2WorldVector(room.gatePositions[exitGateId]);

    var fadeDuration = 0.5;
    var rippleDuration = 0.1;

    double maxDelay = 0;
    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - exitWorldPos).length * rippleDuration;
      sprite.opacity = 1;

      maxDelay = max(maxDelay, fadeDuration + d);
    }

    for (var actor in actorList) {
      double d = (actor.position - exitWorldPos).length * rippleDuration;
      actor.opacity = 1;

      maxDelay = max(maxDelay, fadeDuration + d);
    }

    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - exitWorldPos).length * rippleDuration;

      sprite.add(OpacityEffect.fadeOut(EffectController(duration: fadeDuration, startDelay: maxDelay - d)));
    }

    for (var actor in actorList) {
      double d = (actor.position - exitWorldPos).length * rippleDuration;

      actor.add(OpacityEffect.fadeOut(EffectController(duration: fadeDuration, startDelay: maxDelay - d)));
    }

    add(
      FunctionEffect(
        (_, __) {},
        onComplete: () {
          clean();
          removeFromParent();
        },
        EffectController(duration: maxDelay + fadeDuration + 1),
      ),
    );
  }

  @override
  void onLoad() async {
    buildSpriteGrid(0).then((value) {
      fadeIn();
    });
  }

  List<Component> pendingClean = [];

  void armClean() {
    pendingClean.addAll(tileSpriteGrid.values);
    tileSpriteGrid = {};

    pendingClean.addAll(actorList);
    actorList = [];
  }

  void clean() {
    for (var component in pendingClean) {
      component.removeFromParent();
    }
  }

  Future<void> buildSpriteGrid(double startingOpacity) async {
    armClean();
    List<Future> tileLoadFutures = [];

    for (var pos in await room.getAllPositions()) {
      tileLoadFutures.add(
        Future(() async {
          (String, int)? bgImg = await room.assetAt(p: pos);

          if (bgImg != null) {
            SpriteComponent backgroundTile = SpriteComponent(
              sprite: await Sprite.load(bgImg.$1),
              priority: 0,
              size: Vector2.all(1),
              position: mapPos2WorldVector(pos),
              anchor: Anchor.center,
              angle: bgImg.$2 * pi / 2,
              bleed: 0.01,
            );

            backgroundTile.opacity = startingOpacity;
            add(backgroundTile);
            tileSpriteGrid[pos] = backgroundTile;
          }

          var tile = await room.at(p: pos);

          if (tile is Tile_Gate) {
            int gateId = room.gatePosToId[pos]!;
            GateDestination? destination = room.gateDestinations[gateId];

            bool usedEntrance = gateId == entranceGateId;

            if (destination != null) {
              String? label = room.gateLables[gateId];

              var gate = Gate(this, gateId, destination, room.gateDirections[gateId], label, position: mapPos2WorldVector(pos));
              gate.opacity = startingOpacity;
              add(gate);
              actorList.add(gate);
            }

            if (usedEntrance) {
              var entrance = EntranceTmpIcePatch(position: mapPos2WorldVector(pos));
              add(
                FunctionEffect(
                  (_, _) {},
                  EffectController(duration: 1, startDelay: 1),

                  onComplete: () {
                    entrance.removeFromParent();
                  },
                ),
              );
              entrance.opacity = startingOpacity;
              add(entrance);
            }
          } else if (tile is Tile_Box) {
            var box = Box(this, position: mapPos2WorldVector(pos));
            box.opacity = startingOpacity;
            actorList.add(box);
            add(box);
          } else if (tile is Tile_WeakWall) {
            var weakWall = WeakWall(position: mapPos2WorldVector(pos));
            actorList.add(weakWall);
            weakWall.opacity = startingOpacity;
            add(weakWall);
          } else if (tile is Tile_Sign) {
            var sign = Sign(tile.text, 0, position: mapPos2WorldVector(pos), textBoxWidth: tile.width.toDouble(), textBoxheight: tile.height.toDouble(),
            );
            add(sign);
          }
        }),
      );
    }

    await Future.wait(tileLoadFutures);

    clean();
  }

  Future<bool> canMove(Vector2 og, Vector2 dst, Direction dir, bool firstPush) async {
    Tile ogTile = await getTile(og);

    if (ogTile is Tile_Stop && !firstPush) {
      return false;
    }

    if (await ogTile.stopsPlayerDuringGameplay()) {
      return true;
    }

    Tile dstTile = await getTile(dst);
    var canWalk = !(await dstTile.stopsPlayerDuringGameplay());

    if (canWalk == true) {
      for (var actor in actorList) {
        if (await worldVector2MapPos(actor.position) == await worldVector2MapPos(dst)) {
          canWalk &= !actor.colision;
        }
      }
    }

    return canWalk;
  }

  Future<bool> canBoxWalkInto(Vector2 dst, Direction dir) async {
    Tile dstTile = await getTile(dst);
    var canWalk = !(await dstTile.stopsBoxDuringGameplay());

    if (canWalk == true) {
      for (var actor in actorList) {
        if (await worldVector2MapPos(actor.position) == await worldVector2MapPos(dst)) {
          canWalk &= !actor.colision;
        }
      }
    }

    return canWalk;
  }

  Future<bool> hit(Vector2 pos, Direction dir, {bool box = false}) async {
    var consecuences = false;
    for (var actor in actorList) {
      if (await worldVector2MapPos(actor.position) == await worldVector2MapPos(pos)) {
        if (!box || actor is Box) {
          consecuences |= await actor.hit(dir);
        }
      }
    }
    return consecuences;
  }

  Future<Tile> getTile(Vector2 worldPos) async {
    try {
      Pos localPos = await worldVector2MapPos(worldPos);

      return room.at(p: localPos);
    } catch (_) {
      return Tile.outside();
    }
  }

  void predictedHit(Vector2 og, Vector2 pos, Direction dir) async {
    for (var actor in actorList) {
      if (await worldVector2MapPos(actor.position) == await worldVector2MapPos(pos)) {
        actor.predictedHit(og, dir);
      }
    }
  }
}

// TODO optimize await worldVector2MapPos(actor.position) == await worldVector2MapPos(pos)
