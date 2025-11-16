import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/BoardDescriptionChains/easy.dart';
import 'package:icedash/BoardDescriptionChains/extreme.dart';
import 'package:icedash/BoardDescriptionChains/hard.dart';
import 'package:icedash/BoardDescriptionChains/normal.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

final errorRoom = DartBoard.newLobby(
  serialized: ''' 
# S # 
# # # 
#   # 
# E # 
''',
  gateMetadata: {
    'E'.codeUnitAt(0): GateMetadata.exit(
      destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
    ),
  },
  signText: [("Parece que te has perdido en la mazmorra, vuelve por donde viniste", 3, 1)],
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
  signText: [],
);

Future<DartBoard> endOfGameRoom(String time, String level) async {
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
        destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3)),
        label: "Back to lobby",
      ),
    },
    signText: [("Tardaste $time segundos en completar el nivel $level", 1, 3)],
  );
}

Map<String, (String, Map<int, GateMetadata>, List<(String, int, int)>)> lobbyRooms = {
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
        destination: GateDestination.roomIdWithGate(roomId: "tutorial", gateId: BigInt.from(0), gameMode: "tutorial"),
        label: "Tutorial",
      ),
      'X'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "singleplayer", gateId: BigInt.from(4)),
        label: "Single Player",
      ),
      'M'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.roomIdWithGate(roomId: "multiplayer", gateId: BigInt.from(0)),
        label: "Multi Player",
      ),
    },
    [("Soy un cartel muy largo", 1, 1), ("Soy otro cartel", 3, 1)],
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
        destination: GateDestination.firstAutogen(boardDescription: easy, gameMode: "easy", boardCount: 5),
        label: "Easy",
      ),
      '2'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: normal, gameMode: "easy", boardCount: 5),
        label: "Normal",
      ),
      '3'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: hard, gameMode: "hard", boardCount: 5),
        label: "Hard",
      ),
      '4'.codeUnitAt(0): GateMetadata.exit(
        destination: GateDestination.firstAutogen(boardDescription: extreme, gameMode: "expert", boardCount: 5),
        label: "Extreme",
      ),
    },
    [],
  ),
};

class RoomTraversal {
  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: BigInt.from(3));
  }

  double start = 0;
  String? gameMode;

  Future<DartBoard> getRoom(GateDestination gateDestination, Direction entryDirection) async {
    if (gateDestination is GateDestination_FirstAutogen) {
      await dartStartSearch(boardDesc: gateDestination.boardDescription, maxBufferedBoards: gateDestination.boardCount);
    }

    if (gateDestination is GateDestination_NextAutoGen || gateDestination is GateDestination_FirstAutogen) {
      AutoGenOutput ret = await dartGetNewBoard(entryDirection: entryDirection);

      if (ret is AutoGenOutput_Ok) {
        if (gateDestination is GateDestination_FirstAutogen) {
          gameMode = gateDestination.gameMode;

          // TODO play audio feedback for starting a new game
          await FlameAudio.play('start_strech.mp3');
          start = DateTime.now().millisecondsSinceEpoch.toDouble();
        } else if (gateDestination is GateDestination_NextAutoGen) {
          await FlameAudio.play('won_room.mp3');
        }

        return ret.field0;
        // TODO track number of rooms and end game after a few
      } else if (ret is AutoGenOutput_NoMoreBufferedBoards) {
        await FlameAudio.play('won_strech.mp3');
        return endOfGameRoom(((DateTime.now().millisecondsSinceEpoch.toDouble() - start) / 1000).toStringAsFixed(2), gameMode!);
      }
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
      await FlameAudio.play('change_room.mp3');

      var roomData = lobbyRooms[gateDestination.roomId];

      if (roomData != null) {
        return DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2, signText: roomData.$3);
      } else {
        return errorRoom;
      }
    }

    throw UnimplementedError();
  }
}
