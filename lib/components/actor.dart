import 'dart:async';

import 'package:flame/components.dart';
import 'package:icedash/src/rust/api/main.dart';

abstract class Actor extends SpriteComponent {
  String asset;
  bool colision;
  bool selffade;

  Actor(this.asset, {super.position, super.angle, this.colision = true, this.selffade = false}) {
    super.priority = 10;
    super.size = Vector2.all(1);
    super.anchor = Anchor.center;
  }

  Future<bool> hit(Direction dir);

  void predictedHit(Vector2 startOfMovement, Direction dir);


  @override
  FutureOr<void> onLoad() async {
    super.sprite = await Sprite.load(asset);
    await super.onLoad();
  }
}
