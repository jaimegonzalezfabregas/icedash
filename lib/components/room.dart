import 'dart:math';
import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/tiling.dart';

class RoomComponent extends Component {
  Room room;

  late Rect worldBB;
  late Direction exitDirection;

  Set<SpriteComponent> fadeables = {};

  Vector2 entranceWorldPos;
  late Vector2 exitWorldPos;
  late Vector2 resetWorldPos;
  late Vector2 entranceRoomPos;

  RoomComponent(this.entranceWorldPos, Direction entranceDirection, this.room) {
    while (room.getStartDirection() != entranceDirection) {
      room = room.rotateLeft();
    }

    entranceRoomPos = Vector2.array(room.getStart().dartVector());

    var resetRoomPos = Vector2.array(room.getReset().dartVector());
    resetWorldPos = resetRoomPos - entranceRoomPos + entranceWorldPos;

    var exitRoomPos = Vector2.array(room.getEnd().dartVector());
    exitWorldPos = exitRoomPos - entranceRoomPos + entranceWorldPos;

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x,
      entranceWorldPos.y - entranceRoomPos.y,
      room.getWidth().toDouble(),
      room.getHeight().toDouble(),
    );
  }

  void fadeIn() {
    var fade_speed = 0.05;
    var ripple_speed = 0.05;
    for (var sprite in fadeables) {
      double d = (sprite.position - entranceWorldPos).length * ripple_speed;
      sprite.opacity = 0;

      sprite.add(OpacityEffect.fadeIn(EffectController(duration: fade_speed + d, startDelay: d)));
    }
  }

  void fadeOut(onDone) {
    var fade_speed = 0.05;
    var ripple_speed = 0.05;

    double max_delay = 0;
    for (var sprite in fadeables) {
      double d = (sprite.position - exitWorldPos).length * ripple_speed;

      max_delay = max(max_delay, fade_speed + d);
    }
    for (var sprite in fadeables) {
      double d = (sprite.position - exitWorldPos).length * ripple_speed;
      sprite.opacity = 1;

      sprite.add(OpacityEffect.fadeOut(EffectController(duration: max_delay - d + fade_speed, startDelay: max_delay - d)));
    }
    add(FunctionEffect((_, __) => onDone, EffectController(duration: max_delay)));
  }

  Tile? queryMapDisplayTile(List<List<Tile>> tilemap, int x, int y, bool center) {
    if (y >= tilemap.length || y < 0) {
      return null;
    }
    if (x >= tilemap[y].length || x < 0) {
      return null;
    }
    Tile ret = tilemap[y][x];

    if (ret is Tile_Gate) {
      return Tile.ice();
    }

    if (!center) {
      if (ret is Tile_Entrance) {
        return Tile.wall();
      }
    }

    return ret;
  }

  Map<String, Tile?> neighbouring(List<List<Tile>> tilemap, int x, int y) {
    return {
      "center": queryMapDisplayTile(tilemap, x, y, true),
      "north": queryMapDisplayTile(tilemap, x, y - 1, false),
      "south": queryMapDisplayTile(tilemap, x, y + 1, false),
      "east": queryMapDisplayTile(tilemap, x + 1, y, false),
      "west": queryMapDisplayTile(tilemap, x - 1, y, false),
      "northeast": queryMapDisplayTile(tilemap, x + 1, y - 1, false),
      "northwest": queryMapDisplayTile(tilemap, x - 1, y - 1, false),
      "southeast": queryMapDisplayTile(tilemap, x + 1, y + 1, false),
      "southwest": queryMapDisplayTile(tilemap, x - 1, y + 1, false),
    };
  }

  @override
  void onLoad() async {
    for (var (y, row) in room.getMap().field0.indexed) {
      for (var (x, tile) in row.indexed) {
        var neigh = neighbouring(room.getMap().field0, x, y);
        String? bgImg = neigh2Img(neigh);

        if (bgImg != null) {
          SpriteComponent img = SpriteComponent(
            priority: 0,
            size: Vector2.all(1),
            position: Vector2(x.toDouble(), y.toDouble()) - entranceRoomPos + entranceWorldPos,
          );

          img.sprite = await Sprite.load(bgImg);

          add(img);
          fadeables.add(img);
        }

        if (tile is Tile_Entrance) {
          var door = SpriteComponent(
            priority: 0,
            size: Vector2.all(1),
            position: Vector2(x.toDouble(), y.toDouble()) - entranceRoomPos + entranceWorldPos,
          );

          var postNeigh = neigh;
          postNeigh["center"] = Tile.wall();
          String? fgImg = neigh2Img(neigh);
          if (fgImg != null) {
            door.sprite = await Sprite.load(fgImg);

            door.opacity = 0;
            door.add(
              OpacityEffect.fadeIn(
                EffectController(
                  duration: 1,
                  startDelay: 1,
                  onMax: () {
                    fadeables.add(door);
                  },
                ),
              ),
            );
            add(door);
          }
        }
      }
    }
    fadeIn();
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    // TODO migrate to rust
    Tile dstTile = getTile(dst);
    return dstTile is! Tile_Wall && dstTile is! Tile_Entrance;
  }

  Tile getTile(Vector2 worldPos) {
    try {
      Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

      return room.getMap().field0[(localPos.y).round()][(localPos.x).round()];
    } catch (_) {
      return Tile.outside();
    }
  }
}
