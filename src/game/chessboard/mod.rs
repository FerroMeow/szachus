use bevy::prelude::*;

mod components;
mod systems;
use components::*;
use systems::*;

pub struct Chessboard;

impl Plugin for Chessboard {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera)
            .add_systems(Startup, draw_chessboard)
            .add_systems(Startup, add_chess_pieces);
    }
}
