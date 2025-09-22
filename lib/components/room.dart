import 'dart:ui';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/direction.dart';
import 'package:icedash/tile.dart';

class RoomComponent extends Component implements OpacityProvider {
  List<List<Tile>> tileMap;
  @override
  double opacity = 0;

  late Rect worldBB;
  late Direction exitDirection;

  Set<OpacityProvider> wallSet = {};

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

    throw Exception("entrance not in a border $entrance, $tileMap");
  }

  (List<List<Tile>>, (int, int)) rotateMap(
    List<List<Tile>> input,
    (int, int) entrancePos,
  ) {
    int rows = input.length;
    int cols = input[0].length;
    List<List<Tile>> rotated = List.generate(
      cols,
      (_) => List.filled(rows, Tile.ice),
    );

    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < cols; j++) {
        rotated[j][rows - 1 - i] = input[i][j];
      }
    }

    return (rotated, (rows - 1 - entrancePos.$2, entrancePos.$1));
  }

  late Vector2 entranceRoomPos;
  Vector2 entranceWorldPos;

  RoomComponent(
    this.entranceWorldPos,
    Direction entranceDirection,
    (int, int) entranceMapPos,
    this.tileMap,
  ) {
    assert(tileMap.isNotEmpty);

    var (mapEntranceDirection) = analyzeEntrance(entranceMapPos, tileMap);

    while (mapEntranceDirection != entranceDirection) {
      List<List<Tile>> localTileMap;
      (localTileMap, entranceMapPos) = rotateMap(tileMap, entranceMapPos);
      tileMap = localTileMap;
      mapEntranceDirection = analyzeEntrance(entranceMapPos, tileMap);
    }

    entranceRoomPos = Vector2(
      entranceMapPos.$1.toDouble() * 100,
      entranceMapPos.$2.toDouble() * 100,
    );

    worldBB = Rect.fromLTWH(
      entranceWorldPos.x - entranceRoomPos.x,
      entranceWorldPos.y - entranceRoomPos.y,
      tileMap[0].length.toDouble() * 100,
      tileMap.length.toDouble() * 100,
    );
  }

  Tile? queryMap(List<List<Tile>> tilemap, int x, int y) {
    if (y >= tilemap.length || y < 0) {
      return null;
    }
    if (x >= tilemap[y].length || x < 0) {
      return null;
    }
    Tile ret = tilemap[y][x];

    if (ret == Tile.gate) {
      return Tile.ice;
    }
    return ret;
  }

  Map<String, Tile?> neighbouring(List<List<Tile>> tilemap, int x, int y) {
    return {
      "center": queryMap(tilemap, x, y),
      "north": queryMap(tilemap, x, y - 1),
      "south": queryMap(tilemap, x, y + 1),
      "east": queryMap(tilemap, x + 1, y),
      "west": queryMap(tilemap, x - 1, y),
      "northeast": queryMap(tilemap, x + 1, y - 1),
      "northwest": queryMap(tilemap, x - 1, y - 1),
      "southeast": queryMap(tilemap, x + 1, y + 1),
      "southwest": queryMap(tilemap, x - 1, y + 1),
    };
  }

  @override
  void onLoad() async {
    for (var (y, row) in tileMap.indexed) {
      for (var (x, tile) in row.indexed) {
        var neigh = neighbouring(tileMap, x, y);
        String? bgImg = Tile.neigh2Img(neigh);

        if (bgImg != null) {
          SpriteComponent img = SpriteComponent(
            priority: 0,
            size: Vector2.all(101),
            position:
                Vector2(x.toDouble() * 100, y.toDouble() * 100) -
                entranceRoomPos +
                entranceWorldPos,
          );

          img.sprite = await Sprite.load(bgImg);

          add(img);
          wallSet.add(img);
        } else {
          print("bgTilePattens has no $neigh");
        }

        if (tile == Tile.entrance) {
          var door = SpriteComponent(
            priority: 0,
            size: Vector2.all(101),
            position:
                Vector2(x.toDouble() * 100, y.toDouble() * 100) -
                entranceRoomPos +
                entranceWorldPos,
          );

          var postNeigh = neigh;
          postNeigh["center"] = Tile.wall;
          String? fgImg = Tile.neigh2Img(neigh);
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
