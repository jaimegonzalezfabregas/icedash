import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/room_traversal/lobby_map.dart';
import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(RoomIdAndGate(roomId: "lev_0_lobby", gateId: 3));
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
           FlameAudio.play('start_strech.mp3');
          start = DateTime.now().millisecondsSinceEpoch.toDouble();
        } else if (gateDestination is GateDestination_NextAutoGen) {
           FlameAudio.play('won_room.mp3');
        }

        return (ret.field0, 0);
      } else if (ret is AutoGenOutput_NoMoreBufferedBoards) {
         FlameAudio.play('won_strech.mp3');
        return (
          await endOfGameRoom(((DateTime.now().millisecondsSinceEpoch.toDouble() - start) / 1000), endOfGameMetadata!, entryDirection),
          0,
        );
      }
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
       FlameAudio.play('change_room.mp3');

      return lobbyRoom(gateDestination, entryDirection);
    }

    return (await turnRoom(gateDestination, entryDirection), 0);
  }
}
