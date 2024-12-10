use bevy::{
    prelude::{NextState, Res, ResMut},
    tasks::IoTaskPool,
};

use crate::game::{resources::PlayerColorResource, GameState};

use super::{
    resources::WebsocketChannels, GameWsControlMsg, GameWsUpdateMsg, MatchmakingResponse, WsUpdate,
};

pub(crate) fn ws_get_color(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    ws_update: Res<WsUpdate>,
) {
    let Some(GameWsUpdateMsg::Matchmaking(MatchmakingResponse::Success { color })) = ws_update.0
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

pub(crate) fn ws_update(
    websocket_channels: Res<WebsocketChannels>,
    mut ws_update: ResMut<WsUpdate>,
) {
    match websocket_channels.rx_updates.try_recv() {
        Err(_) => ws_update.0 = None,
        Ok(msg) => ws_update.0 = Some(msg),
    };
}
