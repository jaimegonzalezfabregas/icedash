
import 'package:icedash/src/rust/api/main.dart';

String? neigh2Img(Map<String, Tile?> neigh) {
if (neigh["center"] is Tile_Ice) {
    return "ice.png";
}
if (neigh["center"] is Tile_Entrance) {
    return "ice.png";
}
if (neigh["center"] is Tile_Gate) {
    return "ice.png";
}

if (neigh["center"] is Tile_Wall) {
    bool eastNotSolid = neigh["east"] is Tile_Ice || (neigh["east"] is Tile_Wall && neigh["southeast"] is Tile_Ice);

    bool westNotSolid = neigh["west"] is Tile_Ice || (neigh["west"] is Tile_Wall && neigh["southwest"] is Tile_Ice);

    bool northNotSolid = neigh["north"] is Tile_Ice;

    if (neigh["south"] is Tile_Ice) {
        return "wall0.png";
    }

    if (neigh["northeast"] is Tile_Ice && neigh["north"] is Tile_Wall && 
        neigh["east"] is Tile_Wall && neigh["northwest"] is Tile_Ice && 
        neigh["west"] is Tile_Wall) {
        if (neigh["southwest"] is! Tile_Ice && neigh["southeast"] is! Tile_Ice) {
            return "wall12.png";
        }
    }

    if (neigh["northwest"] is Tile_Ice && neigh["north"] is Tile_Wall && 
        neigh["west"] is Tile_Wall && neigh["east"] is Tile_Ice) {
        if (neigh["southwest"] is! Tile_Ice) {
            return "wall13.png";
        }
    }

    if (neigh["northeast"] is Tile_Ice && neigh["north"] is Tile_Wall && 
        neigh["east"] is Tile_Wall && neigh["west"] is Tile_Ice) {
        if (neigh["southeast"] is! Tile_Ice) {
            return "wall14.png";
        }
    }

    if (neigh["northeast"] is Tile_Ice && neigh["north"] is Tile_Wall && 
        neigh["east"] is Tile_Wall) {
        if (neigh["southeast"] is! Tile_Ice) {
            return "wall3.png";
        }
    }

    if (neigh["northwest"] is Tile_Ice && neigh["north"] is Tile_Wall && 
        neigh["west"] is Tile_Wall) {
        if (neigh["southwest"] is! Tile_Ice) {
            return "wall11.png";
        }
    }

    if (neigh["south"] is Tile_Wall && neigh["north"] is Tile_Ice && 
        eastNotSolid && westNotSolid) {
        return "wall8.png";
    }

    if (neigh["south"] is Tile_Wall && eastNotSolid && westNotSolid) {
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

    if (neigh["west"] is Tile_Wall && neigh["north"] is Tile_Wall && 
        neigh["southwest"] is Tile_Ice) {
        return "wall4.png";
    }

    if (northNotSolid) {
        return "wall5.png";
    }
}

  return null;
}
