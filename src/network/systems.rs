use bevy::{
    prelude::{NextState, Res, ResMut},
    tasks::IoTaskPool,
};

use crate::game::{resources::PlayerColorResource, GameState};

use super::{resources::WebsocketChannels, GameWsControlMsg, GameWsUpdateMsg, MatchmakingResponse};

pub(crate) fn ws_get_color(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    websocket_channels: Res<WebsocketChannels>,
) {
    let Ok(GameWsUpdateMsg::Matchmaking(MatchmakingResponse::Success { color })) =
        websocket_channels.rx_updates.try_recv()
    else {
        return;
    };
    player_color.0 = color;
    next_game_state.set(GameState::Playing);
}

pub(crate) fn on_game_start_confirm(websocket_channels: Res<WebsocketChannels>) {
    let tx = websocket_channels.tx_control.clone();
    IoTaskPool::get()
        .spawn(async move { tx.send(GameWsControlMsg::Ack).await.unwrap() })
        .detach();
}
