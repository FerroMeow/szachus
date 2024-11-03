use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod systems;
use systems::*;

use super::{begin_game_as_white, GameState};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ChessPieceTypeEnum {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
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
