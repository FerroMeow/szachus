use bevy::prelude::*;

mod game;
mod main_menu;

pub use game::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(main_menu::MainMenu)
        .add_plugins(game::Game)
        .run();
}
