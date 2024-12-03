use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_mod_picking::prelude::*;

use crate::{
    game::{
        chessboard::components::{ChessBoardTile, ChessPiece},
        ChessPieceColorEnum, TurnState,
    },
    network::{resources::WebsocketChannels, GameWsControlMsg},
};

use super::{
    to::{ChessMove, Column, Position, Row},
    PieceMoveState, SelectedPiece,
};

pub(super) fn handle_pawn_click(
    mut event: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    query: Query<(Entity, &ChessPiece), With<ChessPiece>>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for event in event.read() {
        let PointerButton::Primary = event.button else {
            continue;
        };
        let clicked_entity = event.target;

        let Some(_) = query.iter().find(|(entity, chess_piece)| {
            if *entity != clicked_entity {
                return false;
            }
            if !chess_piece.alive {
                return false;
            }
            let ChessPieceColorEnum::White = chess_piece.color else {
                return false;
            };
            true
        }) else {
            continue;
        };
        selected_piece.0 = Some(clicked_entity);
        next_state.set(PieceMoveState::PieceSelected);
    }
}

pub(super) fn handle_field_click(
    mut ev: EventReader<Pointer<Click>>,
    mut next_move_state: ResMut<NextState<PieceMoveState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut q_pieces: Query<(&mut Transform, &mut ChessPiece), Without<ChessBoardTile>>,
    q_tiles: Query<(&Transform, &ChessBoardTile), Without<ChessPiece>>,
    selected_piece: Res<SelectedPiece>,
    websocket_channels: Res<WebsocketChannels>,
) {
    let Some(selected_piece) = selected_piece.0 else {
        return;
    };
    let piece_vec = q_pieces.iter().map(|(_, piece)| *piece).collect::<Vec<_>>();
    let mut queried_piece = q_pieces.get_mut(selected_piece).unwrap();
    for ev in ev.read() {
        let selected_tile = ev.target;
        let Ok(queried_tile) = q_tiles.get(selected_tile) else {
            continue;
        };
        if !queried_piece
            .1
            .is_move_valid(queried_tile.1, &piece_vec[..])
        {
            continue;
        }
        // Send the turn data to the server, naively assume it's a correct move
        let chess_move = ChessMove {
            position_from: Position {
                row: Row(queried_piece.1.x as u8),
                column: Column(queried_piece.1.y as u8),
            },
            position_to: Position {
                row: Row(queried_tile.1.x as u8),
                column: Column(queried_tile.1.x as u8),
            },
        };
        let tx_current = websocket_channels.tx_control.clone();
        IoTaskPool::get()
            .spawn(async move {
                tx_current
                    .send(GameWsControlMsg::TurnEnd(chess_move))
                    .await
                    .unwrap()
            })
            .detach();
        // Move the piece to the desired location
        queried_piece.0.translation.x = queried_tile.0.translation.x;
        queried_piece.0.translation.y = queried_tile.0.translation.y;
        queried_piece.1.x = queried_tile.1.x;
        queried_piece.1.y = queried_tile.1.y;
        // Stop the turn
        next_move_state.set(PieceMoveState::TurnBeginning);
        next_turn_state.set(TurnState::WaitingTurn);
    }
}
