import 'package:flame/components.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/main.dart';

class WeakWall extends Actor {
  WeakWall({super.position}) : super("weakwall.png");

  @override
  Future<bool> hit(Direction dir) async {
    removeFromParent();
    super.colision = false;
    return true;
  }

  @override
  void predictedHit(Vector2 startOfMovement, Direction dir) {}
}
