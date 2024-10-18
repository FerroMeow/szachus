use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod systems;
use components::*;
use systems::*;

use super::{begin_game_as_white, GameState};

pub(crate) struct ChessboardCoordinates {
    pub horizontal: usize,
    pub vertical: usize,
}

pub(crate) enum ChessPieceTypeEnum {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

impl From<UVec2> for ChessboardCoordinates {
    fn from(value: UVec2) -> Self {
        Self {
            horizontal: value.x as usize,
            vertical: value.y as usize,
        }
    }
}

pub struct Chessboard;

impl Plugin for Chessboard {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera)
            .add_systems(Startup, draw_chessboard)
            .add_systems(Startup, begin_game_as_white.after(draw_chessboard))
            .add_systems(OnEnter(GameState::Playing), add_chess_pieces);
    }
}
