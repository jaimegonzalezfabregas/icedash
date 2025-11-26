import 'package:icedash/main.dart';
import 'package:icedash/room_traversal/lobby_map.dart';
import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';
import 'package:shared_preferences/shared_preferences.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  Future<GateDestination> getOnLoadDestination() async {
    final SharedPreferencesAsync prefs = SharedPreferencesAsync();
    int lastPlayedLevel = await prefs.getInt("lastPlayedLevel") ?? 0;
    return GateDestination.roomIdWithGate(RoomIdAndGate(roomId: "lev_${lastPlayedLevel}_lobby", gateId: 3));
  }

  double start = 0;
  EndOfGameMetadata? endOfGameMetadata;

  Future<(DartBoard, int)> getRoom(GateDestination gateDestination, Direction entryDirection) async {
    if (gateDestination is GateDestination_FirstAutogen) {
      await dartStartSearch(boardDesc: gateDestination.boardDescription, maxBufferedBoards: gateDestination.boardCount);
    }

    if (gateDestination is GateDestination_NextAutoGen || gateDestination is GateDestination_FirstAutogen) {
      AutoGenOutput ret = await dartGetNewBoard(entryDirection: entryDirection);

      if (ret is AutoGenOutput_Ok) {
        if (gateDestination is GateDestination_FirstAutogen) {
          endOfGameMetadata = gateDestination.endOfGameMetadata;

          // TODO play audio feedback for starting a new game
          playAudio('start_strech.mp3');
          start = DateTime.now().millisecondsSinceEpoch.toDouble();
        } else if (gateDestination is GateDestination_NextAutoGen) {
          playAudio('won_room.mp3');
        }

        return (ret.field0, 0);
      } else if (ret is AutoGenOutput_NoMoreBufferedBoards) {
        playAudio('won_strech.mp3');
        return (await endOfGameRoom(((DateTime.now().millisecondsSinceEpoch.toDouble() - start) / 1000), endOfGameMetadata!, entryDirection), 0);
      }
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
      playAudio('change_room.mp3');

      return lobbyRoom(gateDestination, entryDirection);
    }

    return (await turnRoom(gateDestination, entryDirection), 0);
  }
}
