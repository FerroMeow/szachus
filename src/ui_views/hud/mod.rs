use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::game::GameState;

pub struct Hud;

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud);
        app.init_resource::<GameTimeElapsed>();
        app.add_systems(
            StateTransition,
            update_turn_text.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
            FixedUpdate,
            update_time_elapsed
                .run_if(on_timer(Duration::from_secs(1)).and_then(in_state(GameState::Playing))),
        );
        app.add_systems(OnExit(GameState::Playing), despawn_hud);
    }
}
