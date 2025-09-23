
import 'package:icedash/src/rust/api/simple.dart';

String? neigh2Img(Map<String, Tile?> neigh) {
    if (neigh case {"center": Tile.ice}) {
      return "ice.png";
    }
    if (neigh case {"center": Tile.entrance}) {
      return "ice.png";
    }
    if (neigh case {"center": Tile.gate}) {
      return "ice.png";
    }

    if (neigh["center"] == Tile.wall) {
      bool eastNotSolid =
          neigh["east"] == Tile.ice ||
          (neigh["east"] == Tile.wall && neigh["southeast"] == Tile.ice);

      bool westNotSolid =
          neigh["west"] == Tile.ice ||
          (neigh["west"] == Tile.wall && neigh["southwest"] == Tile.ice);

      bool northNotSolid = neigh["north"] == Tile.ice;

      if (neigh case {"south": Tile.ice}) {
        return "wall0.png";
      }

      if (neigh case {
        "northeast": Tile.ice,
        "north": Tile.wall,
        "east": Tile.wall,
        "northwest": Tile.ice,
        "west": Tile.wall,
      }) {
        if (neigh["southwest"] != Tile.ice && neigh["southeast"] != Tile.ice) {
          return "wall12.png";
        }
      }

      if (neigh case {
        "northwest": Tile.ice,
        "north": Tile.wall,
        "west": Tile.wall,
        "east": Tile.ice,
      }) {
        if (neigh["southwest"] != Tile.ice) {
          return "wall13.png";
        }
      }

      if (neigh case {
        "northeast": Tile.ice,
        "north": Tile.wall,
        "east": Tile.wall,
        "west": Tile.ice,
      }) {
        if (neigh["southeast"] != Tile.ice) {
          return "wall14.png";
        }
      }

      if (neigh case {
        "northeast": Tile.ice,
        "north": Tile.wall,
        "east": Tile.wall,
      }) {
        if (neigh["southeast"] != Tile.ice) {
          return "wall3.png";
        }
      }

      if (neigh case {
        "northwest": Tile.ice,
        "north": Tile.wall,
        "west": Tile.wall,
      }) {
        if (neigh["southwest"] != Tile.ice) {
          return "wall11.png";
        }
      }

      if (neigh["south"] == Tile.wall &&
          neigh["north"] == Tile.ice &&
          eastNotSolid &&
          westNotSolid) {
        return "wall8.png";
      }

      if (neigh["south"] == Tile.wall && eastNotSolid && westNotSolid) {
        return "wall7.png";
      }

      if (northNotSolid && westNotSolid) {
        return "wall10.png";
      }

      if (northNotSolid && eastNotSolid) {
        return "wall9.png";
      }

      if (eastNotSolid) {
        return "wall1.png";
      }

      if (westNotSolid) {
        return "wall6.png";
      }

      if (neigh case {
        "west": Tile.wall,
        "north": Tile.wall,
        "southwest": Tile.ice,
      }) {
        return "wall4.png";
      }

      if (northNotSolid) {
        return "wall5.png";
      }
    }
    return null;
  }
