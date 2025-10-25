import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  (DartBoard, BigInt) getOnLoadRoom() {
    var lobby = '''# # # # # # # # # # 
# # #   # # #     # 
#           w     G 
#         # #     # 
#       # # #     # 
#     b     b b   # 
# # E # # # # # # # ''';

    return (DartBoard.newLobby(serialized: lobby, gateMetadata: {'G'.codeUnitAt(0): ("\\next_autogen", BigInt.from(0))}), BigInt.from(0));
  }

  DartBoard getRoom(String roomId, Pos pos) {
    if (roomId == "\\next_autogen") {
      return dartGetNewBoard();
    } else {
      var lobby = '''# # # # # # # # # # 
# # #   # # #     # 
#           w     G 
#         # #     # 
#       # # #     # 
#     b     b b   # 
# # E # # # # # # # ''';

      return DartBoard.newLobby(serialized: lobby, gateMetadata: {'G'.codeUnitAt(0): ("\\next_autogen", BigInt.from(0))});
    }
  }
}
