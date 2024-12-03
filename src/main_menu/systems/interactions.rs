use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    main_menu::components::MainMenuStartButton,
    network::{resources::WebsocketChannels, server_ws_handler, state::ConnectionState},
    JwtToken,
};

pub fn on_click_game_start(
    mut commands: Commands,
    jwt_token: Res<JwtToken>,
    mut connection_state: ResMut<NextState<ConnectionState>>,
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
        connection_state.set(ConnectionState::WebSocket);
    }
}
