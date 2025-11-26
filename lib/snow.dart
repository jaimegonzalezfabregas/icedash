import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/image_composition.dart';

class Snow extends SpriteComponent {
  double lifetime = 1000;
  Snow(startPosition) {
    position = startPosition;
    priority = 100;
  }

  bool fadingOut = false;

  late Vector2 speed;

  @override
  void onLoad() async {
    int i = Random().nextInt(4);
    lifetime = Random().nextDoubleBetween(3, 6);
    super.sprite = await Sprite.load("snow_flakes/$i.png");
    super.size = Vector2(3 / 16, 3 / 16);
    super.opacity = 0;

    double fallingSpeed = Random().nextDoubleBetween(0.7, 1.2);

    speed = Vector2(0, fallingSpeed);

    position.y -= lifetime / 2 * fallingSpeed;

    add(OpacityEffect.fadeIn(LinearEffectController(1)));
  }

  @override
  void update(double dt) {
    if (Random().nextInt(10) < 1) {
      speed.x = (Random().nextInt(3) - 1) / 3;
    }

    position += speed * dt;
    lifetime -= dt;
    if (lifetime < 0 && !fadingOut) {
      fadingOut = true;

      add(
        OpacityEffect.fadeOut(
          LinearEffectController(1),
          onComplete: () {
            removeFromParent();
          },
        ),
      );
    }

    super.update(dt);
  }
}
