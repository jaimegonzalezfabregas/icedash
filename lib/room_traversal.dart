import 'package:icedash/BoardDescriptionChains/easy.dart';
import 'package:icedash/BoardDescriptionChains/extreme.dart';
import 'package:icedash/BoardDescriptionChains/hard.dart';
import 'package:icedash/BoardDescriptionChains/normal.dart';
import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

final errorRoom = DartBoard.newLobby(
  serialized: '''
# # # 
#   # 
# E # 
''',
  gateMetadata: {
    'E'.codeUnitAt(0): GateMetadata.exit(
      destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
    ),
  },
);

final waitRoom = DartBoard.newLobby(
  serialized: '''
# B # 
#   # 
#   # 
#   # 
# E # 
''',
  gateMetadata: {
    'E'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.nextAutoGen()),
    'B'.codeUnitAt(0): GateMetadata.exit(
      destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
    ),
  },
);

final endOfGameRoom = DartBoard.newLobby(
  serialized: '''
# # E # # 
#       # 
#   s   # 
#       # 
# # G # # 
''',
  gateMetadata: {
    'G'.codeUnitAt(0): GateMetadata.exit(
      destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
      label: "Back to lobby",
    ),
  },
);

class RoomTraversal {
  Map<String, (String, Map<int, GateMetadata>)> lobbyRooms = {
    "StartLobby": (
      '''
# # # S # # # 
#           # 
#   #       # 
T     s     M 
#           # 
# #         # 
# # # E # # # 
''',
      {
        'T'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.roomIdWithGate(roomId: "tutorial", gateId: BigInt.from(0)),
          label: "Tutorial",
        ),
        'S'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.roomIdWithGate(roomId: "singleplayer", gateId: BigInt.from(4)),
          label: "Single Player",
        ),
        'M'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.roomIdWithGate(roomId: "multiplayer", gateId: BigInt.from(0)),
          label: "Multi Player",
        ),
      },
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
        'R'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(0)),
        ),
        '1'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(
            boardDescriptionStack: easy,
          ),
          label: "Easy",
        ),
        '2'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(boardDescriptionStack: normal),
          label: "Normal",
        ),
        '3'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(boardDescriptionStack: hard),
          label: "Hard",
        ),
        '4'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(boardDescriptionStack: extreme),
          label: "Extreme",
        ),
      },
    ),
  };

  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3));
  }

  Future<DartBoard> getRoom(GateDestination gateDestination) async {
    if (gateDestination is GateDestination_FirstAutogen) {
      await dartLoadBoardDescriptionStack(boardDescStack: gateDestination.boardDescriptionStack);
    }

    if (gateDestination is GateDestination_NextAutoGen || gateDestination is GateDestination_FirstAutogen) {
      AutoGenOutput ret = await dartGetNewBoard();

      if (ret is AutoGenOutput_Ok) {
        return ret.field0;
      } else if (ret is AutoGenOutput_NoMoreDescriptionsLoaded) {
        return endOfGameRoom;
      } else {
        return waitRoom;
      }
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
      var roomData = lobbyRooms[gateDestination.roomId];

      if (roomData != null) {
        return DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2);
      } else {
        return errorRoom;
      }
    }

    throw UnimplementedError();
  }
}
