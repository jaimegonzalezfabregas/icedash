import 'package:icedash/src/rust/api/main.dart';
import 'package:icedash/src/rust/logic/board.dart';
import 'package:icedash/src/rust/logic/tile_map.dart';

enum RoomType { lobby, game }

class RoomTraversal {
 

  DartBoard getOnLoadRoom() {
    var lobby = '''# # # # # # # # # # 
# # #   w w w     # 
#         w w     G 
#       w w w     # 
# # E # # # # # # # ''';

    return DartBoard.newLobby(serialized:lobby, start: Pos(x:2,y:4), end:Pos(x:9, y:2), startDirection: Direction.north, endDirection: Direction.east );
  }

  DartBoard getNextRoom(Pos pos) {
    return dartGetNewBoard();
  }
}
