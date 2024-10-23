use bevy::prelude::*;

pub(crate) mod chessboard;
pub(crate) mod resources;
pub(crate) mod systems;
pub(crate) mod turn;

use resources::*;
use systems::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) enum ChessPieceColorEnum {
    White,
    Black,
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
            .add_plugins(turn::Turn)
            .add_systems(Startup, begin_game_as_white);
    }
}
