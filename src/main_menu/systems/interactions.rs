use bevy::prelude::*;

use crate::{
    game::{resources::PlayerColorResource, ChessPieceColorEnum, GameState},
    main_menu::components::MainMenuStartButton,
};

pub fn on_click_game_start(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    query: Query<&Interaction, (Changed<Interaction>, With<MainMenuStartButton>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            debug!("Click detected");
            player_color.0 = ChessPieceColorEnum::White;
            next_game_state.set(GameState::Playing);
            return;
        }
    }
}
