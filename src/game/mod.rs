use bevy::prelude::*;

pub(crate) mod chessboard;
pub(crate) mod resources;
pub(crate) mod systems;
pub(crate) mod turn;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub(crate) enum ChessPieceColorEnum {
    #[default]
    White,
    Black,
}

impl ChessPieceColorEnum {
    fn opposite(&self) -> Self {
        match *self {
            ChessPieceColorEnum::White => ChessPieceColorEnum::Black,
            ChessPieceColorEnum::Black => ChessPieceColorEnum::White,
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum GameState {
    #[default]
    NotInGame,
    Playing,
    Finished,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum TurnState {
    #[default]
    PlayersTurn,
    WaitingTurn,
}

pub(crate) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<TurnState>()
            .add_plugins(chessboard::Chessboard)
            .add_plugins(turn::Turn);
    }
}
