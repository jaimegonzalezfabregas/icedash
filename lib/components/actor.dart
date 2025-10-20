import 'dart:async';

import 'package:flame/components.dart';
import 'package:icedash/src/rust/api/main.dart';

abstract class Actor extends SpriteComponent {
  String asset;
  Actor(this.asset, {super.position}) {
    super.priority = 0;
    super.size = Vector2.all(1);
  }

  bool colision = true;
  void hit(Direction dir);

  @override
  FutureOr<void> onLoad() async {
    super.sprite = await Sprite.load(asset);
    await super.onLoad();
  }
}
