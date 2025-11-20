import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:shared_preferences/shared_preferences.dart';

Future<DartBoard> endOfGameRoom(double score, (String, String) level, Direction entranceDirection) async {
  final SharedPreferencesAsync prefs = SharedPreferencesAsync();
  await prefs.setDouble(level.$1, score);

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
        destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: 3),
        label: "Back to lobby",
      ),
    },
    signText: [("Tardaste ${score.toStringAsFixed(2)} segundos en completar el nivel $level", 1, 3)],
    entranceDirection: (BigInt.from(0), entranceDirection),
  );
}

Future<(String, Map<int, GateMetadata>, List<(String, int, int)>, Direction)> lobby_room(int lev) async {
  final SharedPreferencesAsync prefs = SharedPreferencesAsync();

  String score_find_exit_id = "score_for_lev_${lev}_find_exit";
  String score_find_perfect_path_id = "score_for_lev_${lev}_find_perfect_path";

  double? score_find_exit = await prefs.getDouble(score_find_exit_id);
  double? score_find_perfect_path = await prefs.getDouble(score_find_perfect_path_id);

  double max_score_find_exit = 300;
  double max_score_find_perfect_path = 300;

  var mensaje_find_exit = score_find_exit == null
      ? "Sin registros, necesitas una puntuación de menos de $max_score_find_exit s para continuar"
      : "$score_find_exit s / $max_score_find_exit s";
  var mensaje_find_perfect_path = score_find_perfect_path == null
      ? "Sin registros, necesitas una puntuación de menos de $max_score_find_perfect_path s para continuar"
      : "$score_find_perfect_path s / $max_score_find_perfect_path s";

  var l =
      ((score_find_exit ?? max_score_find_exit) < max_score_find_exit &&
          (score_find_perfect_path ?? max_score_find_perfect_path) < max_score_find_perfect_path)
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
        destination: GateDestination.roomIdWithGate(roomId: "lev_${lev}_find_exit", gateId: 0),
        label: "Encuentra la salida",
      ),
      'B'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "lev_${lev}_find_perfect_path", gateId: 0),
        label: "Camino perfecto",
      ),
      'E'.codeUnitAt(0): lev == 0
          ? GateMetadata.entryOnly()
          : GateMetadata.exit(destination: GateDestination.roomIdWithGate(roomId: "lev_${lev - 1}_lobby", gateId: 0)),
      'N'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.roomIdWithGate(roomId: "lev_${lev + 1}_lobby", gateId: 3)),
    },
    [("Nivel $lev", 3, 1), (mensaje_find_exit, 3, 1), (mensaje_find_perfect_path, 3, 1)],
    Direction.north,
  );
}

Future<(String, Map<int, GateMetadata>, List<(String, int, int)>, Direction)> getLobbyRoom(String id) async {
  var matchLobby = RegExp(r'lev_(?<lev>[0-9]+)_lobby').firstMatch(id);
  if (matchLobby != null) {
    return await lobby_room(int.parse(matchLobby.namedGroup("lev")!));
  }

  var matchFindExit = RegExp(r'lev_(?<lev>[0-9]+)_find_exit').firstMatch(id);
  if (matchFindExit != null) {
    throw UnimplementedError();
  }

  var matchPerfectMatch = RegExp(r'lev_(?<lev>[0-9]+)_find_perfect_path').firstMatch(id);
  if (matchPerfectMatch != null) {
    throw UnimplementedError();
  }

  throw UnimplementedError();
}

Future<(DartBoard, int)> lobbyRoom(GateDestination_RoomIdWithGate gate, Direction entryDirection) async {
  var roomData = await getLobbyRoom(gate.roomId);

  DartBoard dest = await DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2, signText: roomData.$3);

  if (dest.gateDirections[gate.gateId] != entryDirection) {
    return (await turnRoom(gate, entryDirection), 0);
  } else {
    return (dest, await gate.getGateId());
  }
}
