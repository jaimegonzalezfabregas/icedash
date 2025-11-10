import 'package:icedash/src/rust/api/main.dart';

import 'package:icedash/BoardDescriptionChains/Interpolator.dart';

final normal = BoardDescriptionInterpolator(
  end: BoardDescription(
    sizeRangeMin: 7,
    sizeRangeMax: 11,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 0,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 1,
    vignetPercentageMin: 5,
    vignetPercentageMax: 10,
  ),

  start: BoardDescription(
    sizeRangeMin: 7,
    sizeRangeMax: 10,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 0,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 0,
    vignetPercentageMin: 5,
    vignetPercentageMax: 10,
  ),
).getStack(10);
