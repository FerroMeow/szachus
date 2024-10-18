use std::default;

use bevy::prelude::*;

pub struct Game;

mod chessboard;

mod systems;

use systems::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    NotInGame,
    Playing,
    Finished,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TurnState {
    #[default]
    PlayersTurn,
    WaitingTurn,
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<TurnState>()
            .add_plugins(chessboard::Chessboard);
    }
}
