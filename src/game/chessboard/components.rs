use bevy::prelude::*;

enum PlayerColor {
    White,
    Black,
}

#[derive(Component)]
struct ChessPieceColor(PlayerColor);

#[derive(Component)]
struct ChessPieceAlive(bool);

#[derive(Component)]
struct Pawn;

#[derive(Component)]
struct Knight;

#[derive(Component)]
struct Rook;

#[derive(Component)]
struct Bishop;

#[derive(Component)]
struct Queen;

#[derive(Component)]
struct King;
