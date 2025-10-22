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
    print("room component entrance direction is $entranceDirection");
    while (room.getStartDirection() != entranceDirection) {
      print("room entrance direction is ${room.getStartDirection()}");

      room = room.rotateLeft();
    }
    print("room entrance direction is ${room.getStartDirection()}");

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
      double d = (actor.position - entranceWorldPos).length * rippleDuration;

      await actor.add(OpacityEffect.fadeIn(EffectController(duration: fadeDuration, startDelay: d)));
    }
  }

  Future fadeOut(onDone) async {
    var fadeDuration = 0.5;
    var rippleDuration = 0.1;

    double maxDelay = 0;
    for (var sprite in tileSpriteGrid.values) {
      double d = (sprite.position - exitWorldPos).length * rippleDuration;
      sprite.opacity = 1;

      maxDelay = max(maxDelay, fadeDuration + d);
    }

    print("max delay $maxDelay");

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

          onDone();
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

    for (var pos in room.getAllPositions()) {
      var tile = room.at(pos: pos);
      String? bgImg = room.neighbourAt(pos: pos).getAsset();

      if (bgImg != null) {
        SpriteComponent img = SpriteComponent(priority: 0, size: Vector2.all(1), position: mapPos2WorldVector(pos));

        img.sprite = await Sprite.load(bgImg);

        add(img);
        tileSpriteGrid[pos] = img;
      }

      if (tile == Tile.entrance) {
        String? asset = room.neighbourAt(pos: pos).maskCenter(tile: Tile.wall).getAsset();

        if (asset != null) {
          var entrance = Entrance(asset, position: mapPos2WorldVector(pos));
          add(
            FunctionEffect(
              (_, _) {},
              EffectController(duration: 1, startDelay: 1),

              onComplete: () {
                actorList.add(entrance);
              },
            ),
          );

          add(entrance);
        }
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

  bool canWalkInto(Vector2 dst) {
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

  bool canBoxWalkInto(Vector2 dst, Direction dir){
    Tile dstTile = getTile(dst);
    var canWalk = !dstTile.stopsBoxDuringGameplay();

    if (canWalk == true) {
      for (var actor in actorList) {
        if (actor.colision && worldVector2MapPos(actor.position) == worldVector2MapPos(dst)) {

          if(actor is Box){
            actor.hit(dir);
          }

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
      Pos localPos = worldVector2MapPos(worldPos);

      return room.at(pos: localPos);
    } catch (_) {
      return Tile.outside;
    }
  }
}
