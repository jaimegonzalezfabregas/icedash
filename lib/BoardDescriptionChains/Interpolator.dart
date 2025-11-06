import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:icedash/src/rust/api/main.dart';

class BoardDescriptionInterpolator {
  BoardDescription start;
  BoardDescription end;

  BoardDescriptionInterpolator({required this.start, required this.end});

  List<BoardDescription> getStack(int length){
    return getChain(length).reversed.toList();
  }

  List<BoardDescription> getChain(int length) {
    List<BoardDescription> ret = [];

    for (var i = 0; i < length; i++) {
      ret.add(getInterpolated(i / (length - 1)));
    }

    return ret;
  }

  BoardDescription getInterpolated(double factor) {
    var start_data = (start.asList()).map((i) => i.toDouble()).toList();
    var end_data = (end.asList()).map((i) => i.toDouble()).toList();

    var interpolated_data = start_data;

    for (var i = 0; i < interpolated_data.length; i++) {
      interpolated_data[i] += (end_data[i] - start_data[i]) * factor;
    }

    return BoardDescription.fromList(data: Int64List.fromList(interpolated_data.map((e) => e.round()).toList()));
  }
}
