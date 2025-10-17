import 'dart:collection';
import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/src/rust/api/main.dart';

class RoomComponent extends Component {
  late Room reset_room;
  Room room;

  late Rect worldBB;
  late Direction exitDirection;

  Vector2 entranceWorldPos;
  late Vector2 exitWorldPos;
  late Vector2 resetWorldPos;
  late Vector2 entranceRoomPos;
  Map<Pos, SpriteComponent> tileSpriteGrid = {};

  Vector2 mapPos2WorldVector(Pos p) {
    return Vector2.array(p.dartVector()) - entranceRoomPos + entranceWorldPos;
  }

  Pos WorldVector2MapPos(Vector2 v) {
    int x = (v.x - entranceWorldPos.x + entranceRoomPos.x).floor();
    int y = (v.y - entranceWorldPos.y + entranceRoomPos.y).floor();
    return Pos(x: x, y: y);
  }

  void reset() {
    room = reset_room;

  
    rebuildSpriteGrid();
  }

  RoomComponent(this.entranceWorldPos, Direction entranceDirection, this.room) {
    reset_room = room;
    while (room.getStartDirection() != entranceDirection) {
      room = room.rotateLeft();
    }

    entranceRoomPos = Vector2.array(room.getStart().dartVector());

    resetWorldPos = mapPos2WorldVector(room.getReset());
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
    await rebuildSpriteGrid();
    fadeIn();
  }

  Future<void> rebuildSpriteGrid() async {
      for (var e in tileSpriteGrid.values) {
      e.removeFromParent();
    }
    tileSpriteGrid = {};


    for (var pos in room.getAllPositions()) {
      var tile = room.at(pos: pos);
      String? bgImg = tile.getAsset();

      if (bgImg != null) {
        SpriteComponent img = SpriteComponent(priority: 0, size: Vector2.all(1), position: mapPos2WorldVector(pos));

        img.sprite = await Sprite.load(bgImg);

        add(img);
        tileSpriteGrid[pos] = img;
      }

      if (tile is Tile_Entrance) {
        var door = SpriteComponent(priority: 0, size: Vector2.all(1), position: mapPos2WorldVector(pos));

        door.sprite = await Sprite.load(Tile.wall().getAsset()!);

        door.opacity = 0;
        door.add(
          OpacityEffect.fadeIn(
            EffectController(
              duration: 1,
              startDelay: 1,
              onMax: () {
                tileSpriteGrid[pos]?.removeFromParent();

                tileSpriteGrid[pos] = door;
              },
            ),
          ),
        );
        add(door);
      }
    }
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    Tile dstTile = getTile(dst);
    var ret = dstTile.stopsPlayer();

    return !ret;
  }

  void hit(Vector2 pos, Direction dir) {
    Tile hitTile = getTile(pos);

    if (hitTile is Tile_WeakWall) {
      setTile(pos, Tile_Ice());
    }
  }

  Tile getTile(Vector2 worldPos) {
    try {
      Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

      return room.getMap().field0[(localPos.y).round()][(localPos.x).round()];
    } catch (_) {
      return Tile.outside();
    }
  }

  void setTile(Vector2 worldPos, Tile t) async {
    Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

    room.set(localPos,t);

    rebuildSpriteGrid();
  }
}
