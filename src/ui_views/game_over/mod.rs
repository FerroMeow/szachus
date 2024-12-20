use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::game::GameState;

pub struct GameOverScreen;

impl Plugin for GameOverScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Finished), spawn)
            .add_systems(OnExit(GameState::Finished), despawn);
    }
}
