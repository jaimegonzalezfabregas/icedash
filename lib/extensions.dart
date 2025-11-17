import 'package:flame/components.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/pos.dart';

extension IntoDartVector4Direction on Direction {
  Vector2 dartVector() {
    if (this == Direction.north) {
      return Vector2(0, -1);
    }
    if (this == Direction.south) {
      return Vector2(0, 1);
    }
    if (this == Direction.east) {
      return Vector2(1, 0);
    }
    if (this == Direction.west) {
      return Vector2(-1, 0);
    }
    throw UnimplementedError("Unreacheable");
  }
}

extension IntoDartVector4Pos on Pos {
  Vector2 dartVector() {
    return Vector2(x.toDouble(), y.toDouble());
  }
}
