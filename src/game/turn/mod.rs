use bevy::prelude::*;

pub(crate) mod systems;
pub(crate) mod to;

use systems::*;

use crate::network::systems::ws_update;

use super::{GameState, TurnState};

pub struct Turn;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) enum PieceMoveState {
    #[default]
    TurnBeginning,
    PieceSelected,
}

#[derive(Resource, Default)]
struct SelectedPiece(pub Option<Entity>);

impl Plugin for Turn {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPiece>()
            .init_state::<PieceMoveState>()
            .add_systems(
                FixedUpdate,
                (ws_get_turn, ws_get_move, ws_get_confirm, ws_get_win)
                    .after(ws_update)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                handle_pawn_click.run_if(
                    in_state(GameState::Playing).and_then(in_state(TurnState::PlayersTurn)),
                ),
            )
            .add_systems(
                Update,
                handle_field_click.run_if(
                    in_state(PieceMoveState::PieceSelected)
                        .and_then(in_state(GameState::Playing))
                        .and_then(in_state(TurnState::PlayersTurn)),
                ),
            );
    }
}
