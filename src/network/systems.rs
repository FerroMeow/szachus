use bevy::prelude::{NextState, Res, ResMut};

use crate::game::{resources::PlayerColorResource, GameState, TurnState};

use super::{
    resources::WebsocketChannels, ChessPieceColorEnum, GameWsUpdateMsg, MatchmakingResponse,
};

pub(crate) fn ws_get_color(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_color: ResMut<PlayerColorResource>,
    mut turn_state: ResMut<NextState<TurnState>>,
    websocket_channels: Res<WebsocketChannels>,
) {
    let Ok(GameWsUpdateMsg::Matchmaking(MatchmakingResponse::Success { color })) =
        websocket_channels.rx_updates.try_recv()
    else {
        return;
    };
    turn_state.set(match color {
        ChessPieceColorEnum::White => TurnState::PlayersTurn,
        ChessPieceColorEnum::Black => TurnState::WaitingTurn,
    });
    player_color.0 = color;
    next_game_state.set(GameState::Playing);
}
