use crate::game::ChessPieceColorEnum;
use bevy::prelude::*;

use super::ChessPieceTypeEnum;

#[derive(Component)]
pub(crate) struct ChessPieceType(pub ChessPieceTypeEnum);

#[derive(Component)]
pub(crate) struct ChessPieceColor(pub ChessPieceColorEnum);

#[derive(Component)]
pub(crate) struct ChessPieceAlive(pub bool);

#[derive(Component)]
pub(crate) struct Pawn;

#[derive(Component)]
pub(crate) struct Knight;

#[derive(Component)]
pub(crate) struct Rook;

#[derive(Component)]
pub(crate) struct Bishop;

#[derive(Component)]
pub(crate) struct Queen;

#[derive(Component)]
pub(crate) struct King;
