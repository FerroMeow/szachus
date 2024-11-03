use crate::game::ChessPieceColorEnum;
use bevy::prelude::*;

use super::ChessPieceTypeEnum;

#[derive(Component)]
pub(crate) struct ChessBoardTile {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub(crate) struct ChessPiece {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub(crate) struct ChessPieceType(pub ChessPieceTypeEnum);

#[derive(Component)]
pub(crate) struct ChessPieceColor(pub ChessPieceColorEnum);

#[derive(Component)]
pub(crate) struct ChessPieceAlive(pub bool);

#[derive(Component)]
pub(crate) struct MainCamera;
