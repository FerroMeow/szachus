use bevy::prelude::*;

mod components;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::game::resources::PlayerColorResource;
use crate::game::GameState;
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NotInGame), spawn_menu)
            .init_resource::<PlayerColorResource>()
            .add_systems(
                Update,
                on_click_game_start.run_if(in_state(GameState::NotInGame)),
            )
            .add_systems(OnEnter(GameState::Playing), despawn_menu);
    }
}
