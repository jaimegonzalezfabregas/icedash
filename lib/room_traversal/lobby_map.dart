import 'package:icedash/BoardDescriptionChains/easy.dart';
import 'package:icedash/BoardDescriptionChains/extreme.dart';
import 'package:icedash/BoardDescriptionChains/hard.dart';
import 'package:icedash/BoardDescriptionChains/normal.dart';
import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';

Map<String, (String, Map<int, GateMetadata>, List<(String, int, int)>, Direction)> lobbyRooms = {
  "StartLobby": (
    '''
# # # X # # # 
# S         # 
#   #       # 
T     s     M 
#           # 
# #   S     # 
# # # E # # # 
''',
    {
      'T'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "tutorial", gateId: 0, gameMode: "tutorial"),
        label: "Tutorial",
      ),
      'X'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "singleplayer", gateId: 4),
        label: "Single Player",
      ),
      'M'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "multiplayer", gateId: 0),
        label: "Multi Player",
      ),
    },
    [("Soy un cartel muy largo", 1, 1), ("Soy otro cartel", 3, 1)],
    Direction.north,
  ),

  "singleplayer": (
    '''
# # # # # # # 
#           # 
3     s     4 
#           # 
1     s     2 
#           # 
# # # R # # # 
''',
    {
      'R'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: 0)),
      '1'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: easy, gameMode: "easy", boardCount: 1),
        label: "Easy",
      ),
      '2'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: normal, gameMode: "easy", boardCount: 1),
        label: "Normal",
      ),
      '3'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: hard, gameMode: "hard", boardCount: 1),
        label: "Hard",
      ),
      '4'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: extreme, gameMode: "expert", boardCount: 1),
        label: "Extreme",
      ),
    },
    [],
    Direction.north,
  ),
};

Future<(DartBoard, int)> lobbyRoom(GateDestination_RoomIdWithGate gate, Direction entryDirection) async {
  var roomData = lobbyRooms[gate.roomId];

  if (roomData != null) {
    DartBoard dest = await DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2, signText: roomData.$3);

    if (dest.gateDirections[gate.gateId] != entryDirection) {
      return (await turnRoom(gate, entryDirection), 0);
    } else {
      return (dest, await gate.getGateId());
    }
  } else {
    return (await errorRoom(entryDirection), 0);
  }
}
