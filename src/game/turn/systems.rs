use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{
    chessboard::components::{ChessBoardTile, ChessPiece, ChessPieceAlive, ChessPieceColor},
    ChessPieceColorEnum,
};

use super::{PieceMoveState, SelectedPiece};

pub(super) fn handle_pawn_click(
    mut event: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    query: Query<(Entity, &ChessPieceColor, &ChessPieceAlive), With<ChessPiece>>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for event in event.read() {
        let PointerButton::Primary = event.button else {
            continue;
        };
        let clicked_entity = event.target;

        let Some(piece) = query.iter().find(|(entity, color, alive)| {
            if *entity != clicked_entity {
                return false;
            }
            let ChessPieceAlive(true) = alive else {
                return false;
            };
            let ChessPieceColor(ChessPieceColorEnum::White) = color else {
                return false;
            };
            true
        }) else {
            continue;
        };
        selected_piece.0 = Some(clicked_entity);
        next_state.set(PieceMoveState::PieceSelected);
        debug!("Clicked on entity {:?}", piece.0);
    }
}

pub(super) fn handle_field_click(
    mut ev: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    mut q_pieces: Query<(&mut Transform, &mut ChessPiece), Without<ChessBoardTile>>,
    q_tiles: Query<(&Transform, &ChessBoardTile), Without<ChessPiece>>,
    selected_piece: Res<SelectedPiece>,
) {
    let Some(selected_piece) = selected_piece.0 else {
        return;
    };
    let mut queried_piece = q_pieces.get_mut(selected_piece).unwrap();
    for ev in ev.read() {
        let selected_tile = ev.target;
        let Ok(queried_tile) = q_tiles.get(selected_tile) else {
            continue;
        };
        debug!(
            "Moving entity {:?} to position {:?}",
            selected_piece,
            queried_tile.0.translation.truncate()
        );
        queried_piece.0.translation.x = queried_tile.0.translation.x;
        queried_piece.0.translation.y = queried_tile.0.translation.y;
        queried_piece.1.x = queried_tile.1.x;
        queried_piece.1.y = queried_tile.1.y;
        next_state.set(PieceMoveState::TurnBeginning);
    }
}
