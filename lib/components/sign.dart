import 'dart:async';

import 'package:flame/components.dart';
import 'package:flame/text.dart';
import 'package:flutter/material.dart';
import 'package:flutter/painting.dart';


final bigNum = 100.0;

class Sign extends Component {
  String text;
  double angle;
  Sign(this.text, this.angle);

  @override
  FutureOr<void> onLoad() {
    for (var x in [
      (Color.fromARGB(255, 0, 0, 0), Vector2(0, -1)),
      (Color.fromARGB(255, 0, 0, 0), Vector2(1, 0)),
      (Color.fromARGB(255, 0, 0, 0), Vector2(0, 1)),
      (Color.fromARGB(255, 0, 0, 0), Vector2(-1, 0)),
      (Color.fromARGB(255, 255, 255, 255), Vector2(0, 0)),
    ]) {
      Future(() {
        add(MyTextBox(text, delta: x.$2, color: x.$1, size: Vector2(1, 1), position: Vector2(0.5, 0.5), angle: angle));
      });
    }

    return super.onLoad();
  }
}

class MyTextBox extends TextBoxComponent {
  Color color;

  MyTextBox(String text, {required Vector2 size, required this.color, required Vector2 delta, required position, super.angle})
    : super(
        position: position + delta / 16 / 4,
        scale: Vector2.all(1 / bigNum),
        size: size * bigNum,
        text: text,
        textRenderer: TextPaint(
          style: TextStyle(fontSize: bigNum / 4, color: color, fontFamily: "BoldPixels"),
        ),
        boxConfig: TextBoxConfig(maxWidth: 3, margins: EdgeInsets.all(0)),
        align: Anchor.center,
        anchor: Anchor.center,
      );
}
