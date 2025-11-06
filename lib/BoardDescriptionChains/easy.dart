import 'package:icedash/BoardDescriptionChains/Interpolator.dart';
import 'package:icedash/src/rust/api/main.dart';

final easy = BoardDescriptionInterpolator(
  end: BoardDescription(
    sizeRangeMin: 7,
    sizeRangeMax: 14,
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
    sizeRangeMin: 7,
    sizeRangeMax: 10,
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
