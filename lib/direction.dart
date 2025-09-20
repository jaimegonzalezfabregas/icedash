import 'package:flame/game.dart';

enum Direction { north, south, east, west }

extension DirectionToVector on Direction {
  Vector2 get vector {
    switch (this) {
      case Direction.north:
        return Vector2(0, -1);
      case Direction.south:
        return Vector2(0, 1);
      case Direction.east:
        return Vector2(-1, 0);
      case Direction.west:
        return Vector2(1, 0);
    }
  }
}
