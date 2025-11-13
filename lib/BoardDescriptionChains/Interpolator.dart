import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:icedash/src/rust/api/main.dart';

class BoardDescriptionInterpolator {
  BoardDescription start;
  BoardDescription end;

  BoardDescriptionInterpolator({required this.start, required this.end});

  Future<List<BoardDescription>> getStack(int length) async {
    return (await getChain(length)).reversed.toList();
  }

  Future<List<BoardDescription>> getChain(int length) async {
    List<BoardDescription> ret = [];

    for (var i = 0; i < length; i++) {
      ret.add(await getInterpolated(i / (length - 1)));
    }

    return ret;
  }

  Future<BoardDescription> getInterpolated(double factor) async {
    var startData = (await start.asList()).map((i) => i.toDouble()).toList();
    var endData = (await end.asList()).map((i) => i.toDouble()).toList();

    var interpolatedData = startData;

    for (var i = 0; i < interpolatedData.length; i++) {
      interpolatedData[i] += (endData[i] - startData[i]) * factor;
    }

    return BoardDescription.fromList(data: Int64List.fromList(interpolatedData.map((e) => e.round()).toList()));
  }
}
