import 'package:flame/components.dart';
import 'package:icedash/components/actor.dart';
import 'package:icedash/components/room.dart';
import 'package:icedash/src/rust/api/main.dart';

class Box extends Actor {
  RoomComponent room;
  Box(this.room, {super.position}) : super("box.png");

  @override
  void hit(Direction dir) {
    while (room.canWalkInto(position, position + Vector2.array(dir.dartVector()))) {
      position = position + Vector2.array(dir.dartVector());
    }
  }
}
