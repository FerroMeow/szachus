use bevy::prelude::*;

pub mod fatal_error;
pub mod game_over;
pub mod hud;
pub mod main_menu;
pub mod retry_game;

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            fatal_error::FatalErrorScreen,
            game_over::GameOverScreen,
            hud::Hud,
            main_menu::MainMenu,
            retry_game::RetryPlugin,
        ));
    }
}
