import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/main.dart';

class WeakWall extends Actor {
  WeakWall({super.position}) : super("weakwall.png");
  
  @override
  void hit(Direction dir) {
    removeFromParent();
    super.colision = false;
  }


}