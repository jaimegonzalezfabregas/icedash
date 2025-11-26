import 'dart:async';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:icedash/src/rust/api/direction.dart';

abstract class Actor extends PositionComponent implements OpacityProvider {
  String? asset;
  bool colision;
  bool selffade;
  SpriteComponent? display;

  Actor(
    this.asset, {
    super.position,
    super.angle,
    this.colision = true,
    this.selffade = false,
  }) {
    super.priority = 10;
    super.size = Vector2.all(1);
    super.anchor = Anchor.center;
  }

  Future<bool> hit(Direction dir);

  void predictedHit(Vector2 startOfMovement, Direction dir);

  @override
  FutureOr<void> onLoad() async {
    if (asset != null) {
      display = SpriteComponent(
        sprite: await Sprite.load(asset!),
        size: Vector2(1, 1),
      );
      add(display!);
    }
    await super.onLoad();
  }

  @override
  set opacity(double value) {
    if (display != null) {
      display!.opacity = value;
    }
  }

  @override
  double get opacity => display?.opacity ?? 1;
}
