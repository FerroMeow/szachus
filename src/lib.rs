use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use wasm_bindgen::prelude::*;

mod game;
mod network;
mod ui_views;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct JwtToken {
    pub jwt: String,
}

impl Default for JwtToken {
    fn default() -> Self {
        let Some(window) = web_sys::window() else {
            panic!("Browser window not found")
        };
        let Ok(Some(storage)) = window.local_storage() else {
            panic!("Local storage not found");
        };
        let Ok(Some(jwt)) = storage.get("jwt") else {
            panic!("JWT not found");
        };
        JwtToken { jwt }
    }
}

#[wasm_bindgen]
pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,szachus=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                }),
        )
        .add_plugins(DefaultPickingPlugins)
        .init_resource::<JwtToken>()
        .add_plugins(ui_views::retry_game::RetryPlugin)
        .add_plugins(ui_views::fatal_error::FatalErrorScreen)
        .add_plugins(network::Network)
        .add_plugins(ui_views::main_menu::MainMenu)
        .add_plugins(game::Game)
        .add_plugins(ui_views::game_over::GameOverScreen)
        .run();
}
