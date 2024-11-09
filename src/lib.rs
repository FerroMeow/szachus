use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use wasm_bindgen::prelude::*;

mod game;
mod main_menu;

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
        .add_plugins(main_menu::MainMenu)
        .add_plugins(game::Game)
        .run();
}
