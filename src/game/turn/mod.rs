use bevy::prelude::*;

pub(crate) mod systems;

use systems::*;

pub struct Turn;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum PieceMoveState {
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
            .add_systems(Update, handle_pawn_click)
            .add_systems(
                Update,
                handle_field_click.run_if(in_state(PieceMoveState::PieceSelected)),
            );
    }
}
