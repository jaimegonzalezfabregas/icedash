
import 'package:icedash/src/rust/api/dart_board.dart';
import 'package:icedash/src/rust/api/direction.dart';
import 'package:icedash/src/rust/api/main.dart';

Future<DartBoard> errorRoom(Direction entranceDirection) {
  return DartBoard.newLobby(
    serialized: ''' 
# # # # # 
#   S   # 
# # # # # 
# #   # # 
# # E # # 
''',
    gateMetadata: {'E'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: 3))},
    signText: [("Parece que te has perdido en la mazmorra, vuelve por donde viniste", 3, 1)],
    entranceDirection: (BigInt.from(0), entranceDirection),
  );
}

Future<DartBoard> waitRoom(Direction entranceDirection) {
  return DartBoard.newLobby(
    serialized: '''
# B # 
#   # 
#   # 
#   # 
# E # 
''',
    gateMetadata: {
      'E'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.nextAutoGen()),
      'B'.codeUnitAt(0): GateMetadata.exit(destination: GateDestination.roomIdWithGate(roomId: "StartLobby", gateId: 3)),
    },
    signText: [],
    entranceDirection: (BigInt.from(0), entranceDirection),
  );
}

Future<DartBoard> turnRoom(GateDestination gateDestination, Direction entranceDirection) async {
  return DartBoard.newLobby(
    serialized: '''
# E # # 
#   # # 
#     G 
# # # # 
''',
    gateMetadata: {'G'.codeUnitAt(0): GateMetadata.exit(destination: gateDestination)},
    signText: [],
    entranceDirection: (BigInt.from(0), entranceDirection),
  );
}
