use bevy::prelude::{NextState, Res, ResMut};

use crate::game::{resources::PlayerColorResource, GameState};

use super::{resources::WebsocketChannels, GameWsUpdateMsg, MatchmakingResponse};

pub(crate) fn ws_get_color(
    websocket_channels: Res<WebsocketChannels>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
) {
    let Ok(GameWsUpdateMsg::Matchmaking(MatchmakingResponse::Success { color })) =
        websocket_channels.rx_updates.try_recv()
    else {
        return;
    };
    player_color.0 = color;
    next_game_state.set(GameState::Playing);
}
