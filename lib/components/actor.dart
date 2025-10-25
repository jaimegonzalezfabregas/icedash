import 'dart:async';

import 'package:flame/components.dart';
import 'package:icedash/src/rust/api/main.dart';

abstract class Actor extends SpriteComponent {
  String asset;
  bool colision;

  Actor(this.asset, {super.position, super.angle, this.colision = true}) {
    super.priority = 10;
    super.size = Vector2.all(1);
    super.anchor = Anchor.center;
  }

  bool hit(Direction dir);

  @override
  FutureOr<void> onLoad() async {
    super.sprite = await Sprite.load(asset);
    await super.onLoad();
  }
}
