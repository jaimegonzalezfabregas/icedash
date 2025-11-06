import 'package:icedash/src/rust/api/main.dart';

import 'package:icedash/BoardDescriptionChains/Interpolator.dart';

final extreme = BoardDescriptionInterpolator(
  end: BoardDescription(
    sizeRangeMin: 15,
    sizeRangeMax: 30,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 0,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 0,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),

  start: BoardDescription(
    sizeRangeMin: 15,
    sizeRangeMax: 25,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 0,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 0,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),
).getStack(5);
