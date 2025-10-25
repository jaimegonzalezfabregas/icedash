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

    var gateMetadata = {'G'.codeUnitAt(0): ("\\next_autogen", BigInt.from(0))};
    var ret = DartBoard.newLobby(serialized: lobby, gateMetadata: gateMetadata);

    return (ret, BigInt.from(1));
  }

  DartBoard getRoom(String roomId, Pos pos) {
    if (roomId == "\\next_autogen") {
      return dartGetNewBoard();
    } else {
      return getOnLoadRoom().$1;
    }
  }
}
