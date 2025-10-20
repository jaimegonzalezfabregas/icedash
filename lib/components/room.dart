import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/actors/box.dart';
import 'package:icedash/components/actors/entrance.dart';
import 'package:icedash/components/actors/weak_wall.dart';
import 'package:icedash/src/rust/api/main.dart';

class RoomComponent extends Component {
  Room room;

  late Rect worldBB;
  late Direction exitDirection;

  Vector2 entranceWorldPos;
  late Vector2 exitWorldPos;
  late Vector2 entranceRoomPos;
  Map<Pos, SpriteComponent> tileSpriteGrid = {};
  List<Actor> actorList = [];
  Direction entranceDirection;

  Vector2 mapPos2WorldVector(Pos p) {
    return Vector2.array(p.dartVector()) - entranceRoomPos + entranceWorldPos;
  }

  Pos worldVector2MapPos(Vector2 v) {
    int x = (v.x - entranceWorldPos.x + entranceRoomPos.x).round();
    int y = (v.y - entranceWorldPos.y + entranceRoomPos.y).round();
    return Pos(x: x, y: y);
  }

  void reset() {
    buildSpriteGrid();
  }

  RoomComponent(this.entranceWorldPos, this.entranceDirection, this.room) {
    while (room.getStartDirection() != entranceDirection) {
      room = room.rotateLeft();
    }

    entranceRoomPos = Vector2.array(room.getStart().dartVector());

    exitWorldPos = mapPos2WorldVector(room.getEnd());

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x,
      entranceWorldPos.y - entranceRoomPos.y,
      room.getWidth().toDouble(),
      room.getHeight().toDouble(),
    );
  }

  Future fadeIn() async {
    var fadeSpeed = 0.05;
    var rippleSpeed = 0.05;
    for (var sprite in tileSpriteGrid.values) {
      sprite.opacity = 0;
    }

    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - entranceWorldPos).length * rippleSpeed;

      await sprite.add(OpacityEffect.fadeIn(EffectController(duration: fadeSpeed + d, startDelay: d)));
    }
  }

  Future fadeOut(onDone) async {
    var fadeSpeed = 0.05;
    var rippleSpeed = 0.05;

    double maxDelay = 0;
    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - exitWorldPos).length * rippleSpeed;
      sprite.opacity = 1;

      maxDelay = max(maxDelay, fadeSpeed + d);
    }
    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - exitWorldPos).length * rippleSpeed;

      await sprite.add(OpacityEffect.fadeOut(EffectController(duration: maxDelay - d + fadeSpeed, startDelay: maxDelay - d)));
    }
    add(FunctionEffect((_, __) => onDone, EffectController(duration: maxDelay)));
  }

  @override
  void onLoad() async {
    await buildSpriteGrid();
    fadeIn();
  }

  Future<void> buildSpriteGrid() async {
    for (var e in tileSpriteGrid.values) {
      e.removeFromParent();
    }

    for (var actor in actorList) {
      actor.removeFromParent();
    }

    tileSpriteGrid = {};
    actorList = [];

    for (var pos in room.getAllPositions()) {
      var tile = room.at(pos: pos);
      String? bgImg = tile.getAsset();

      if (bgImg != null) {
        SpriteComponent img = SpriteComponent(priority: 0, size: Vector2.all(1), position: mapPos2WorldVector(pos));

        img.sprite = await Sprite.load(bgImg);

        add(img);
        tileSpriteGrid[pos] = img;
      }

      if (tile == Tile.entrance) {
        var entrance = Entrance(position: mapPos2WorldVector(pos));
        actorList.add(entrance);
        add(entrance);
      }

      if (tile == Tile.box) {
        var box = Box(this, position: mapPos2WorldVector(pos));
        actorList.add(box);
        add(box);
      }

      if (tile == Tile.weakWall) {
        var weakWall = WeakWall(position: mapPos2WorldVector(pos));
        actorList.add(weakWall);
        add(weakWall);
      }
    }
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    Tile dstTile = getTile(dst);
    var canWalk = !dstTile.stopsPlayerDuringGameplay();

    if (canWalk == true) {
      for (var actor in actorList) {
        if (actor.colision && worldVector2MapPos(actor.position) == worldVector2MapPos(dst)) {
          return false;
        }
      }
    }

    return canWalk;
  }

  void hit(Vector2 pos, Direction dir) {

    for (var actor in actorList) {
      if (worldVector2MapPos(actor.position) == worldVector2MapPos(pos)) {
        actor.hit(dir);
      }
    }
  }

  Tile getTile(Vector2 worldPos) {
    try {
      Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

      return room.getMap().field0[(localPos.y).round()][(localPos.x).round()];
    } catch (_) {
      return Tile.outside;
    }
  }
}
