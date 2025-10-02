import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  List<List<Tile>> decode(String input) {
    // Split the input string into lines
    List<String> lines = input.split('\n');

    // Initialize the lobby list
    List<List<Tile>> lobby = [];

    for (String line in lines) {
      // Trim the line to remove any leading or trailing whitespace
      line = line.trim();

      if (line.isEmpty) {
        continue;
      }

      // Initialize a row for the current line
      List<Tile> row = [];

      for (int i = 0; i < line.length; i++) {
        String tileChar = line[i];

        // Map the character representation to Tile enum values
        switch (tileChar) {
          case '#':
            row.add(Tile.wall());
            break;
          case ' ':
            row.add(Tile.ice());
            break;
          case 'E':
            row.add(Tile.entrance());
            break;
          case 'G':
            row.add(Tile.gate());
            break;
          default:
            throw Exception('Unknown tile character: $tileChar');
        }
      }

      // Add the row to the lobby
      lobby.add(row);
    }

    return lobby;
  }

  Board getOnLoadRoom() {
    var lobby = '''
##########
###      #
#        G
#        #
##E#######
''';

    return Board(map: decode(lobby), start: (2, 4), resetPos: (2,2), end: (9, 2), startDirection: Direction.north, endDirection: Direction.west);
  }

  Board getNextRoom((int, int) pos) {
    return searchBoard();
  }
}
