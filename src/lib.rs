use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use game::chessboard::systems::{BOARD_SIZE, TILE_SIZE};
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
    let log_plugin = LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,szachus=debug".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
    };
    let window = Window {
        title: String::from("Szachuś"),
        resolution: (
            TILE_SIZE * BOARD_SIZE as f32 * 1.5,
            TILE_SIZE * BOARD_SIZE as f32,
        )
            .into(),
        ..default()
    };
    let window_plugin = WindowPlugin {
        primary_window: Some(window),
        ..default()
    };
    let default_plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(log_plugin)
        .set(window_plugin);
    let mut app = App::new();
    app.add_plugins(default_plugins)
        .add_plugins(DefaultPickingPlugins)
        .init_resource::<JwtToken>()
        .add_plugins(ui_views::UI)
        .add_plugins(network::Network)
        .add_plugins(game::Game);
    app.run();
}
