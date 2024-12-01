use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    game::{resources::PlayerColorResource, ChessPieceColorEnum, GameState},
    main_menu::components::MainMenuStartButton,
    network::{resources::WebsocketChannels, server_ws_handler},
    JwtToken,
};

pub fn on_click_game_start(
    mut commands: Commands,
    jwt_token: Res<JwtToken>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    query: Query<&Interaction, (Changed<Interaction>, With<MainMenuStartButton>)>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            return;
        }
        let (tx_control, rx_control) = async_channel::unbounded();
        let (tx_updates, rx_updates) = async_channel::unbounded();
        // memory clones for async move
        IoTaskPool::get()
            .spawn(server_ws_handler(
                jwt_token.jwt.clone(),
                rx_control,
                tx_updates,
            ))
            .detach();
        commands.insert_resource(WebsocketChannels {
            tx_control,
            rx_updates,
        });
        player_color.0 = ChessPieceColorEnum::White;
        next_game_state.set(GameState::Playing);
    }
}
