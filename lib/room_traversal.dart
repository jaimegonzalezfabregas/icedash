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
# G # 
#   # 
# E # 
''',
  gateMetadata: {'E'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.nextAutoGen())},
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
          destination: GateDestination.firstAutogen(profile: []), label: "easy"
        ),
        '2'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []), label: "normal"
        ),
        '3'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []), label: "hard"
        ),
        '4'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []), label: "extreme"
        ),
      },
    ),
  };

  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3));
  }

  Future<DartBoard> getRoom(GateDestination gateDestination) async{
    if (gateDestination is GateDestination_NextAutoGen) {
      var ret = await dartGetNewBoard();

      return ret ?? waitRoom;
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
      var roomData = lobbyRooms[gateDestination.roomId];

      if (roomData != null) {
        return DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2);
      } else {
        return errorRoom;
      }
    } else if (gateDestination is GateDestination_FirstAutogen) {
      dartLoadBoardDescriptionStack(boardDescStack: gateDestination.profile);
      var ret = await dartGetNewBoard();

      return ret ?? waitRoom;
    } 

    throw UnimplementedError();
  }
}
