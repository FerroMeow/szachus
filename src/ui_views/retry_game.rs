use bevy::prelude::*;

use crate::{
    game::{
        resources::{GameWinner, PlayerColorResource},
        turn::{PieceMoveState, SelectedPiece},
        GameState, TurnState,
    },
    network::{
        resources::{WebsocketChannels, WsUpdate},
        state::ConnectionState,
    },
};

use super::hud::resources::GameTimeElapsed;

pub struct RetryPlugin;

impl Plugin for RetryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, reset_game_state);
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct RetryBtn;

pub fn reset_game_state(
    mut commands: Commands,
    q_retry_button: Query<&Interaction, With<RetryBtn>>,
    mut s_connection: ResMut<NextState<ConnectionState>>,
    mut s_game: ResMut<NextState<GameState>>,
    mut s_piece_move: ResMut<NextState<PieceMoveState>>,
    mut s_turn: ResMut<NextState<TurnState>>,
) {
    let Ok(btn_interact) = q_retry_button.get_single() else {
        // Button not interacted with
        return;
    };
    let Interaction::Pressed = *btn_interact else {
        // Button not clicked
        return;
    };
    commands.remove_resource::<WebsocketChannels>();
    commands.remove_resource::<GameWinner>();
    commands.insert_resource(PlayerColorResource::default());
    commands.insert_resource(WsUpdate::default());
    commands.insert_resource(SelectedPiece::default());
    commands.insert_resource(GameTimeElapsed::default());
    s_game.set(GameState::default());
    s_connection.set(ConnectionState::default());
    s_turn.set(TurnState::default());
    s_piece_move.set(PieceMoveState::default());
}
