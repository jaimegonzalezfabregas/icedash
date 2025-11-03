import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/actors/box.dart';
import 'package:icedash/components/actors/entrance.dart';
import 'package:icedash/components/actors/gate.dart';
import 'package:icedash/components/actors/weak_wall.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/src/rust/logic/pos.dart';

class RoomComponent extends Component {
  DartBoard room;

  late Direction exitDirection;

  Vector2 entranceWorldPos;
  late Vector2 entranceRoomPos;
  Map<Pos, SpriteComponent> tileSpriteGrid = {};
  List<Actor> actorList = [];
  Direction entranceDirection;

  BigInt entranceGateId;

  Vector2 mapPos2WorldVector(Pos p) {
    return Vector2.array(pos2DartVector(p: p)) - entranceRoomPos + entranceWorldPos;
  }

  Pos worldVector2MapPos(Vector2 v) {
    int x = (v.x - entranceWorldPos.x + entranceRoomPos.x).round();
    int y = (v.y - entranceWorldPos.y + entranceRoomPos.y).round();
    return Pos(x: x, y: y);
  }

  void reset() {
    buildSpriteGrid();
  }

  RoomComponent(this.entranceWorldPos, this.entranceDirection, this.room, this.entranceGateId) {
    while (room.getGateDirection(gateId: entranceGateId) != entranceDirection) {
      room = room.rotateLeft();
    }

    entranceRoomPos = Vector2.array(pos2DartVector(p: room.getGatePosition(gateId: entranceGateId)));
  }

  Future getWorldBB() async {
    return Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x - 0.5,
      entranceWorldPos.y - entranceRoomPos.y - 0.5,
      (await room.getWidth()).toDouble(),
      (await room.getHeight()).toDouble(),
    );
  }

  Future fadeIn() async {
    var fadeDuration = 0.5;
    var rippleDuration = 0.1;
    for (var sprite in tileSpriteGrid.values) {
      sprite.opacity = 0;
    }

    for (var sprite in actorList) {
      sprite.opacity = 0;
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
  }

  Future fadeOut(BigInt exitGateId) async {
    Vector2 exitWorldPos = mapPos2WorldVector(room.getGatePosition(gateId: exitGateId));

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

      await sprite.add(OpacityEffect.fadeOut(EffectController(duration: fadeDuration, startDelay: maxDelay - d)));
    }

    for (var actor in actorList) {
      double d = (actor.position - exitWorldPos).length * rippleDuration;

      await actor.add(OpacityEffect.fadeOut(EffectController(duration: fadeDuration, startDelay: maxDelay - d)));
    }

    add(
      FunctionEffect(
        (_, __) {},
        onComplete: () {
          clean();
          removeFromParent();
        },
        EffectController(duration: maxDelay + fadeDuration),
      ),
    );
  }

  @override
  void onLoad() async {
    await buildSpriteGrid();
    fadeIn();
  }

  void clean() {
    for (var e in tileSpriteGrid.values) {
      e.removeFromParent();
    }

    for (var actor in actorList) {
      actor.removeFromParent();
    }
  }

  Future<void> buildSpriteGrid() async {
    clean();

    tileSpriteGrid = {};
    actorList = [];

    for (var pos in await room.getAllPositions()) {
      (String, int)? bgImg = await room.assetAt(p: pos);

      if (bgImg != null) {
        SpriteComponent img = SpriteComponent(
          priority: 0,
          size: Vector2.all(1),
          position: mapPos2WorldVector(pos),
          anchor: Anchor.center,
          angle: bgImg.$2 * pi / 2,
        );

        img.sprite = await Sprite.load(bgImg.$1);

        add(img);
        tileSpriteGrid[pos] = img;
      }

      var tile = await room.at(p: pos);

      if (tile is Tile_Gate) {
        BigInt gateId = (await room.getGateIdByPos(p: pos))!;
        GateDestination? destination = await room.getGateDestination(gateId: gateId);

        bool usedEntrance = gateId == entranceGateId;

        if (destination != null) {
          String? label = await room.getGateLabel(gateId: gateId);

          var gate = Gate(this, gateId, destination, room.getGateDirection(gateId: gateId), label, position: mapPos2WorldVector(pos));
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

          add(entrance);
        }
      }

      if (tile is Tile_Box) {
        var box = Box(this, position: mapPos2WorldVector(pos));
        actorList.add(box);
        add(box);
      }

      if (tile is Tile_WeakWall) {
        var weakWall = WeakWall(position: mapPos2WorldVector(pos));
        actorList.add(weakWall);
        add(weakWall);
      }
    }
  }

  Future<bool> canWalkInto(Vector2 og, Vector2 dst, Direction dir, bool userPush) async {
    Tile ogTile = await getTile(og);

    if (ogTile is Tile_Stop && !userPush) {
      return false;
    }

    if (await ogTile.stopsPlayerDuringGameplay()) {
      return true;
    }

    Tile dstTile = await getTile(dst);
    var canWalk = !(await dstTile.stopsPlayerDuringGameplay());

    if (canWalk == true) {
      for (var actor in actorList) {
        if (worldVector2MapPos(actor.position) == worldVector2MapPos(dst)) {
          canWalk &= !actor.colision;

          if (actor is Gate) {
            actor.hit(dir);
          }
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
        if (worldVector2MapPos(actor.position) == worldVector2MapPos(dst)) {
          canWalk &= !actor.colision;

          if (actor is Box) {
            actor.hit(dir);
          }
        }
      }
    }

    return canWalk;
  }

  Future<bool> hit(Vector2 pos, Direction dir) async {
    var consecuences = false;
    for (var actor in actorList) {
      if (worldVector2MapPos(actor.position) == worldVector2MapPos(pos)) {
        consecuences |= await actor.hit(dir);
      }
    }
    return consecuences;
  }

  Future<Tile> getTile(Vector2 worldPos) async {
    try {
      Pos localPos = worldVector2MapPos(worldPos);

      return room.at(p: localPos);
    } catch (_) {
      return Tile.outside();
    }
  }
}
