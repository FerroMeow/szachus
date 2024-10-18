use bevy::prelude::*;

use super::{GameState, PlayerColorResource};

pub(crate) fn begin_game_as_white(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    commands.insert_resource(PlayerColorResource(super::ChessPieceColorEnum::Black));
    state.set(GameState::Playing);
}
