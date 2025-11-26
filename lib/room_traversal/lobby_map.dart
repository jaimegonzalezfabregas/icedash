import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/board_description.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:shared_preferences/shared_preferences.dart';

Future<DartBoard> endOfGameRoom(
  double score,
  EndOfGameMetadata endOfGameMetadata,
  Direction entranceDirection,
) async {
  final SharedPreferencesAsync prefs = SharedPreferencesAsync();
  await prefs.setDouble(endOfGameMetadata.bestScoreId, score);

  return DartBoard.newLobby(
    serialized: '''
# # E # # 
#       # 
# S s   # 
#       # 
# # G # # 
''',
    gateMetadata: {
      'G'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination_RoomIdWithGate(
          endOfGameMetadata.returnGate,
        ),
        label: "Back to lobby",
      ),
    },
    signText: [
      (
        "Tardaste ${score.toStringAsFixed(2)} segundos en completar el nivel ${endOfGameMetadata.level}",
        1,
        3,
      ),
    ],
    entranceDirection: (BigInt.from(0), entranceDirection),
  );
}

Future<(String, Map<int, GateMetadata>, List<(String, int, int)>, Direction)>
lobbyRoomByLev(int lev) async {
  final SharedPreferencesAsync prefs = SharedPreferencesAsync();

  String scoreFindExitId = "score_for_lev_${lev}_find_exit";
  String scoreFindPerfectPathId = "score_for_lev_${lev}_find_perfect_path";

  double? scoreFindExit = await prefs.getDouble(scoreFindExitId);
  double? scoreFindPerfectPath = await prefs.getDouble(scoreFindPerfectPathId);

  double maxScoreFindExit = 300;
  double maxScoreFindPerfectPath = 300;

  var mensajeFindExit = scoreFindExit == null
      ? "Sin registros, necesitas una puntuación de menos de $maxScoreFindExit s para continuar"
      : "$scoreFindExit s / $maxScoreFindExit s";
  var mensajeFindPerfectPath = scoreFindPerfectPath == null
      ? "Sin registros, necesitas una puntuación de menos de $maxScoreFindPerfectPath s para continuar"
      : "$scoreFindPerfectPath s / $maxScoreFindPerfectPath s";

  var l =
      ((scoreFindExit ?? maxScoreFindExit) < maxScoreFindExit &&
          (scoreFindPerfectPath ?? maxScoreFindPerfectPath) <
              maxScoreFindPerfectPath)
      ? "  "
      : "l ";

  return (
    '''
# # # # N # # # # 
#       $l      # 
#       S       # 
A       s       B 
#   S       S   # 
#               # 
# # # # E # # # # 
''',
    {
      'A'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(
          boardDescription: BoardDescription(
            area: (7 * 7) + lev * 5,
            weakWallsPercentageMin: 0,
            weakWallsPercentageMax: 0,
            pilarsPercentageMin: 5,
            pilarsPercentageMax: 10,
            boxPercentageMin: 0,
            boxPercentageMax: 0,
            vignetPercentageMin: 10,
            vignetPercentageMax: 15,
            gameMode: GameMode.findExit,
          ),
          boardCount: 1 + lev,
          endOfGameMetadata: EndOfGameMetadata(
            gamemodeDesc: "Encuentra la salida",
            bestScoreId: scoreFindExitId,
            level: lev,
            returnGate: RoomIdAndGate(roomId: "lev_${lev}_lobby", gateId: 1),
          ),
        ),
        label: "Encuentra la salida",
      ),
      'B'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(
          boardDescription: BoardDescription(
            area: (7 * 7) + lev * 5,
            weakWallsPercentageMin: 0,
            weakWallsPercentageMax: 5,
            pilarsPercentageMin: 0,
            pilarsPercentageMax: 5,
            boxPercentageMin: 0,
            boxPercentageMax: 3,
            vignetPercentageMin: 10,
            vignetPercentageMax: 15,
            gameMode: GameMode.findPerfectPath,
          ),
          boardCount: 1 + lev,
          endOfGameMetadata: EndOfGameMetadata(
            gamemodeDesc: "Camino perfecto",
            bestScoreId: scoreFindPerfectPathId,
            level: lev,
            returnGate: RoomIdAndGate(roomId: "lev_${lev}_lobby", gateId: 2),
          ),
        ),
        label: "Camino perfecto",
      ),
      'E'.codeUnitAt(0): lev == 0
          ? GateMetadata.entryOnly()
          : GateMetadata.exit(
              destination: GateDestination.roomIdWithGate(
                RoomIdAndGate(roomId: "lev_${lev - 1}_lobby", gateId: 0),
              ),
            ),
      'N'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(
          RoomIdAndGate(roomId: "lev_${lev + 1}_lobby", gateId: 3),
        ),
      ),
    },
    [
      ("Nivel $lev", 3, 1),
      (mensajeFindExit, 3, 1),
      (mensajeFindPerfectPath, 3, 1),
    ],
    Direction.north,
  );
}

Future<(String, Map<int, GateMetadata>, List<(String, int, int)>, Direction)>
getLobbyRoom(String id) async {
  var matchLobby = RegExp(r'lev_(?<lev>[0-9]+)_lobby').firstMatch(id);
  if (matchLobby != null) {
    int lev = int.parse(matchLobby.namedGroup("lev")!);

    final SharedPreferencesAsync prefs = SharedPreferencesAsync();
    await prefs.setInt("lastPlayedLevel", lev);

    return await lobbyRoomByLev(lev);
  }

  throw UnimplementedError();
}

Future<(DartBoard, int)> lobbyRoom(
  GateDestination_RoomIdWithGate gate,
  Direction entryDirection,
) async {
  var roomData = await getLobbyRoom(gate.field0.roomId);

  DartBoard dest = await DartBoard.newLobby(
    serialized: roomData.$1,
    gateMetadata: roomData.$2,
    signText: roomData.$3,
  );

  if (dest.gateDirections[gate.field0.gateId] != entryDirection) {
    return (await turnRoom(gate, entryDirection), 0);
  } else {
    return (dest, await gate.getGateId());
  }
}
