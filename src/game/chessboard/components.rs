use crate::game::ChessPieceColorEnum;
use bevy::prelude::*;

use super::ChessPieceTypeEnum;

#[derive(Component)]
pub(crate) struct ChessBoardTile;

#[derive(Component)]
pub(crate) struct ChessPiece;

#[derive(Component)]
pub(crate) struct ChessPieceType(pub ChessPieceTypeEnum);

#[derive(Component)]
pub(crate) struct ChessPieceColor(pub ChessPieceColorEnum);

#[derive(Component)]
pub(crate) struct ChessPieceAlive(pub bool);

#[derive(Component)]
pub(crate) struct MainCamera;
