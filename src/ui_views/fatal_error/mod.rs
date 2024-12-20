use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::game::GameState;

pub struct FatalErrorScreen;

impl Plugin for FatalErrorScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Error), spawn)
            .add_systems(OnExit(GameState::Error), despawn);
    }
}
