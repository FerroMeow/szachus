use bevy::prelude::*;

pub(crate) mod components;
pub(crate) mod systems;
use systems::*;

use super::GameState;

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
            .add_systems(
                OnEnter(GameState::Playing),
                (draw_chessboard, add_chess_pieces),
            )
            .add_systems(OnEnter(GameState::Finished), clean_chessboard);
    }
}
