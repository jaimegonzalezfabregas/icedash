import 'dart:math';

import 'package:flame/components.dart';
import 'package:flame/effects.dart';
import 'package:flame/image_composition.dart';
import 'package:flame/particles.dart';
import 'package:flutter/material.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/main.dart';
import 'package:icedash/src/rust/api/direction.dart';

class PixelParticle extends Particle {
  final Paint paint;
  final double side;

  PixelParticle({required this.paint, this.side = 2 / 16, super.lifespan});

  @override
  void render(Canvas canvas) {
    canvas.drawRect(Rect.fromCenter(center: Offset(0, 0), width: side, height: side), paint);
  }
}

class WeakWall extends Actor {
  final double explosionEffectTime = 2;
  final double speedupFactor = 3;

  WeakWall({super.position}) : super("weakwall.png");

  @override
  Future<bool> hit(Direction dir) async {
    super.colision = false;
    super.selffade = true;

    super.display?.removeFromParent();

    playAudio("hit_weak_wall.mp3");

    add(
      ParticleSystemComponent(
        particle: Particle.generate(
          count: 50,
          lifespan: explosionEffectTime,
          generator: (i) {
            Vector2 p = (Vector2.random() - Vector2.random()) / 2;

            Paint paint;

            switch (i % 11) {
              case 0:
              case 1:
                paint = Paint()..color = Color.fromARGB(255, 57, 74, 97);

              case 2:
              case 3:
                paint = Paint()..color = Color.fromARGB(255, 113, 147, 192);
                break;
              case 4:
                paint = Paint()..color = Color.fromARGB(255, 173, 199, 233);
                break;
              case 5:
              case 6:
              case 7:
              case 8:
                paint = Paint()..color = Color.fromARGB(255, 255, 255, 255);
                break;
              default:
                paint = Paint()..color = Color.fromARGB(255, 180, 207, 243);
                break;
            }

            return AcceleratedParticle(
              lifespan: explosionEffectTime,

              position: Vector2.all(0.5) + p,
              speed: (p * speedupFactor) * Random().nextDoubleBetween(1, 1.5),
              child: ScalingParticle(
                lifespan: explosionEffectTime,
                to: 0,
                curve: Curves.linearToEaseOut,
                child: PixelParticle(paint: paint),
              ),
            );
          },
        ),
      ),
    );

    add(
      FunctionEffect(
        (_, __) {},
        LinearEffectController(explosionEffectTime),
        onComplete: () {
          removeFromParent();
        },
      ),
    );

    return true;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
