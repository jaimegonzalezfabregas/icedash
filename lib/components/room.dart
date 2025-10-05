import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/tiling.dart';

class RoomComponent extends Component implements OpacityProvider {
  Board board;
  @override
  double opacity = 0;

  late Rect worldBB;
  late Direction exitDirection;

  Set<OpacityProvider> wallSet = {};

  Vector2 entranceWorldPos;
  late Vector2 resetWorldPos;
  late Vector2 entranceRoomPos;

  RoomComponent(this.entranceWorldPos, Direction entranceDirection, this.board) {
    while (board.startDirection != entranceDirection) {
      board = board.rotateLeft();
    }

    entranceRoomPos = Vector2.array(board.start.dartVector());
    var resetRoomPos = Vector2.array(board.resetPos.dartVector());
    resetWorldPos = resetRoomPos - entranceRoomPos + entranceWorldPos;

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x,
      entranceWorldPos.y - entranceRoomPos.y,
      board.getWidth().toDouble(),
      board.getHeight().toDouble(),
    );
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
    for (var (y, row) in board.map.field0.indexed) {
      for (var (x, tile) in row.indexed) {
        var neigh = neighbouring(board.map.field0, x, y);
        String? bgImg = neigh2Img(neigh);

        if (bgImg != null) {
          SpriteComponent img = SpriteComponent(
            priority: 0,
            size: Vector2.all(1),
            position: Vector2(x.toDouble(), y.toDouble()) - entranceRoomPos + entranceWorldPos,
          );

          img.sprite = await Sprite.load(bgImg);

          add(img);
          wallSet.add(img);
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
                    wallSet.add(door);
                  },
                ),
              ),
            );
            add(door);
          }
        }
      }
    }
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    // TODO migrate to rust
    Tile dstTile = getTile(dst);
    return dstTile is! Tile_Wall && dstTile is! Tile_Entrance;
  }

  Tile getTile(Vector2 worldPos) {
    try {
      Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

      return board.map.field0[(localPos.y).round()][(localPos.x).round()];
    } catch (_) {
      return Tile.outside();
    }
  }

  @override
  void update(double dt) {
    for (var w in wallSet) {
      w.opacity = opacity;
    }

    super.update(dt);
  }
}
