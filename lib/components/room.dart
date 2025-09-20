import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/components/wall.dart';
import 'package:icedash/direction.dart';
import 'package:icedash/tile.dart';

class RoomComponent extends Component implements OpacityProvider {
  List<List<Tile>> tileMap;
  @override
  double opacity = 0;

  late Rect worldBB;
  late Direction exitDirection;

  Set<Wall> wallSet = {};

  Direction analyzeEntrance((int, int) entrance, List<List<Tile>> tileMap) {
    var width = tileMap[0].length;
    var height = tileMap.length;

    if (entrance.$1 == 0) {
      return Direction.west;
    }
    if (entrance.$1 == width - 1) {
      return Direction.east;
    }
    if (entrance.$2 == 0) {
      return Direction.south;
    }
    if (entrance.$2 == height - 1) {
      return Direction.north;
    }

    throw Exception("entrance not in a border ${entrance}, ${tileMap}");
  }

  (List<List<Tile>>, (int, int)) rotateMap(List<List<Tile>> input, (int, int) entrancePos) {
    int rows = input.length;
    int cols = input[0].length;
    List<List<Tile>> rotated = List.generate(cols, (_) => List.filled(rows, Tile.ice));

    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < cols; j++) {
        rotated[j][rows - 1 - i] = input[i][j];
      }
    }

    return (rotated, (rows - 1 - entrancePos.$2, entrancePos.$1));
  }

  late Vector2 entranceRoomPos;
  Vector2 entranceWorldPos;

  RoomComponent(this.entranceWorldPos, Direction entranceDirection, (int, int) entranceMapPos, this.tileMap) {
    assert(tileMap.isNotEmpty);

    var (mapEntranceDirection) = analyzeEntrance(entranceMapPos, tileMap);

    while (mapEntranceDirection != entranceDirection) {
      List<List<Tile>> localTileMap;
      (localTileMap, entranceMapPos) = rotateMap(tileMap, entranceMapPos);
      tileMap = localTileMap;
      mapEntranceDirection = analyzeEntrance(entranceMapPos, tileMap);
    }

    entranceRoomPos = Vector2(entranceMapPos.$1.toDouble() * 100, entranceMapPos.$2.toDouble() * 100);

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x,
      entranceWorldPos.y - entranceRoomPos.y,
      tileMap[0].length.toDouble() * 100,
      tileMap.length.toDouble() * 100,
    );
  }

  @override
  void onLoad() async {
    for (var (y, row) in tileMap.indexed) {
      for (var (x, tile) in row.indexed) {
        switch (tile) {
          case Tile.entrance:
            var door = Wall(position: Vector2(x.toDouble() * 100, y.toDouble() * 100) - entranceRoomPos + entranceWorldPos);
            door.opacity = 0;
            door.add(
              OpacityEffect.fadeIn(
                EffectController(
                  duration: 0.3,
                  startDelay: 0.5,
                  onMax: () {
                    wallSet.add(door);
                  },
                ),
              ),
            );
            add(door);

            break;
          case Tile.gate:
            break;
          case Tile.wall:
            Wall wall = Wall(position: Vector2(x.toDouble() * 100, y.toDouble() * 100) - entranceRoomPos + entranceWorldPos);

            add(wall);
            wallSet.add(wall);

            break;
          case Tile.ice:
            break;
          case Tile.ground:
            break;
          case Tile.outside:
            break;
        }
      }
    }
  }

  bool canWalkInto(Vector2 origin, Vector2 dst) {
    Tile dstTile = getTile(dst);
    return dstTile != Tile.wall && dstTile != Tile.entrance;
  }

  Tile getTile(Vector2 worldPos) {
    try {
      Vector2 localPos = worldPos - entranceWorldPos + entranceRoomPos;

      return tileMap[(localPos.y / 100).round()][(localPos.x / 100).round()];
    } catch (_) {
      return Tile.outside;
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
