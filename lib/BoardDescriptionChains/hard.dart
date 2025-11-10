import 'package:icedash/src/rust/api/main.dart';

import 'package:icedash/BoardDescriptionChains/Interpolator.dart';

final hard = BoardDescriptionInterpolator(
  end: BoardDescription(
    sizeRangeMin: 10,
    sizeRangeMax: 15,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 4,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 1,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),

  start: BoardDescription(
    sizeRangeMin: 7,
    sizeRangeMax: 15,
    weakWallsPercentageMin: 0,
    weakWallsPercentageMax: 3,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 0,
    boxPercentageMax: 1,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),
).getStack(10);
