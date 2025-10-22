import 'package:icedash/components/actor.dart';
import 'package:icedash/src/rust/api/main.dart';

class WeakWall extends Actor {
  WeakWall({super.position}) : super("weakwall.png");
  
  @override
  bool hit(Direction dir) {
    removeFromParent();
    super.colision = false;
    return true;
  }


}