import 'package:icedash/src/rust/api/main.dart';

import 'package:icedash/BoardDescriptionChains/Interpolator.dart';

final hard = BoardDescriptionInterpolator(
  end: BoardDescription(
    sizeRangeMin: 10,
    sizeRangeMax: 15,
    weakWallsPercentageMin: 2,
    weakWallsPercentageMax: 4,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 2,
    boxPercentageMax: 5,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),

  start: BoardDescription(
    sizeRangeMin: 7,
    sizeRangeMax: 15,
    weakWallsPercentageMin: 2,
    weakWallsPercentageMax: 3,
    pilarsPercentageMin: 2,
    pilarsPercentageMax: 5,
    boxPercentageMin: 2,
    boxPercentageMax: 5,
    vignetPercentageMin: 10,
    vignetPercentageMax: 15,
  ),
).getStack(10);
