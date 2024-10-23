use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(crate) mod systems;

use systems::*;

pub struct Turn;

impl Plugin for Turn {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_click.run_if(input_just_pressed(MouseButton::Left)),
        );
    }
}
