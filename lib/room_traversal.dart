import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/src/rust/logic/pos.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  Map<String, (String, Map<int, (String, BigInt, String?)>)> lobbyRooms = {
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
        'T'.codeUnitAt(0): ("tutorial", BigInt.from(0), "Tutorial"),
        'S'.codeUnitAt(0): ("singleplayer", BigInt.from(0), "Single Player"),
        'M'.codeUnitAt(0): ("multiplayer", BigInt.from(0), "Multi Player"),
      },
    ),
  };

  (String, BigInt) getOnLoadDestination() {
    return ("StartLobby", BigInt.from(3));
  }

  DartBoard getRoom(String roomId, Pos pos) {
    if (roomId == "\\next_autogen") {
      return dartGetNewBoard();
    } else {
      var roomData = lobbyRooms[roomId];

      if (roomData != null) {
        return DartBoard.newLobby(serialized: roomData.$1, gateMetadata: roomData.$2);
      } else {
        return DartBoard.newLobby(
          serialized: '''
# # # 
#   # 
# E # 
''',
          gateMetadata: {'E'.codeUnitAt(0): ("StartLobby", BigInt.from(3), null)},
        );
      }
    }
  }
}
