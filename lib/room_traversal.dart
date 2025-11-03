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
#   # 
#   # 
#   # 
#   # 
# s # 
#   # 
#   # 
#   # 
#   # 
#   # 
#   # 
#   # 
# E # 
''',
  gateMetadata: {
    'E'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.nextAutoGen(), label: "Continue waiting",),
    'B'.codeUnitAt(0): GateMetadata.exit(
      destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
      label: "Go to lobby",
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
            profile: [
              BoardDescription(
                sizeRangeMin: 7,
                sizeRangeMax: 10,
                weakWallsPercentageMin: 0,
                weakWallsPercentageMax: 0,
                pilarsPercentageMin: 10,
                pilarsPercentageMax: 30,
                boxPercentageMin: 0,
                boxPercentageMax: 0,
                vignetPercentageMin: 20,
                vignetPercentageMax: 30,
              ),
              BoardDescription(
                sizeRangeMin: 7,
                sizeRangeMax: 10,
                weakWallsPercentageMin: 0,
                weakWallsPercentageMax: 0,
                pilarsPercentageMin: 10,
                pilarsPercentageMax: 30,
                boxPercentageMin: 0,
                boxPercentageMax: 0,
                vignetPercentageMin: 20,
                vignetPercentageMax: 30,
              ),
              BoardDescription(
                sizeRangeMin: 7,
                sizeRangeMax: 10,
                weakWallsPercentageMin: 0,
                weakWallsPercentageMax: 0,
                pilarsPercentageMin: 10,
                pilarsPercentageMax: 30,
                boxPercentageMin: 0,
                boxPercentageMax: 0,
                vignetPercentageMin: 20,
                vignetPercentageMax: 30,
              ),
              BoardDescription(
                sizeRangeMin: 7,
                sizeRangeMax: 10,
                weakWallsPercentageMin: 0,
                weakWallsPercentageMax: 0,
                pilarsPercentageMin: 10,
                pilarsPercentageMax: 30,
                boxPercentageMin: 0,
                boxPercentageMax: 0,
                vignetPercentageMin: 20,
                vignetPercentageMax: 30,
              ),
              BoardDescription(
                sizeRangeMin: 7,
                sizeRangeMax: 10,
                weakWallsPercentageMin: 0,
                weakWallsPercentageMax: 0,
                pilarsPercentageMin: 10,
                pilarsPercentageMax: 30,
                boxPercentageMin: 0,
                boxPercentageMax: 0,
                vignetPercentageMin: 20,
                vignetPercentageMax: 30,
              ),
            ],
          ),
          label: "Easy",
        ),
        '2'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []),
          label: "Normal",
        ),
        '3'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []),
          label: "Hard",
        ),
        '4'.codeUnitAt(0): GateMetadata.exit(
          destination: GateDestination.firstAutogen(profile: []),
          label: "Extreme",
        ),
      },
    ),
  };

  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3));
  }

  Future<DartBoard> getRoom(GateDestination gateDestination) async {
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
      print("a");

      await dartLoadBoardDescriptionStack(boardDescStack: gateDestination.profile);
      print("b");

      var ret = await dartGetNewBoard();
      print("c");

      return ret ?? waitRoom;
    }

    throw UnimplementedError();
  }
}
