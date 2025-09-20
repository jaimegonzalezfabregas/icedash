import 'package:flame/components.dart';


class Wall extends SpriteComponent {
  Wall({super.position}) : super(size: Vector2.all(100), anchor: Anchor.topLeft);

  @override
  Future<void> onLoad() async {
    sprite = await Sprite.load('Wall_0.png');
  }


}
