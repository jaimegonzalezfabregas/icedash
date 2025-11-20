import 'package:flame_audio/flame_audio.dart';
import 'package:icedash/BoardDescriptionChains/easy.dart';
import 'package:icedash/BoardDescriptionChains/extreme.dart';
import 'package:icedash/BoardDescriptionChains/hard.dart';
import 'package:icedash/BoardDescriptionChains/normal.dart';
import 'package:icedash/room_traversal/lobby_map.dart';
import 'package:icedash/room_traversal/single_rooms.dart';
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';

enum RoomType { lobby, game }

class RoomTraversal {
  GateDestination getOnLoadDestination() {
    return GateDestination.roomIdWithGate(roomId: "lev_0_lobby", gateId: 3);
  }

  double start = 0;
  (String, String)? gameMode;

  Future<(DartBoard, int)> getRoom(GateDestination gateDestination, Direction entryDirection) async {
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

        return (ret.field0, 0);
      } else if (ret is AutoGenOutput_NoMoreBufferedBoards) {
        await FlameAudio.play('won_strech.mp3');
        return (
          await endOfGameRoom(((DateTime.now().millisecondsSinceEpoch.toDouble() - start) / 1000), gameMode!, entryDirection),
          0,
        );
      }
    } else if (gateDestination is GateDestination_RoomIdWithGate) {
      await FlameAudio.play('change_room.mp3');

      return lobbyRoom(gateDestination, entryDirection);
    }

    return (await turnRoom(gateDestination, entryDirection), 0);
  }
}
