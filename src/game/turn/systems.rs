use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_mod_picking::prelude::*;

use crate::{
    game::{
        chessboard::{
            components::{ChessBoardTile, ChessPiece},
            systems::TILE_SIZE,
        },
        resources::PlayerColorResource,
        ChessPieceColorEnum, TurnState,
    },
    network::{
        resources::{WebsocketChannels, WsUpdate},
        GameMessage, GameWsControlMsg, GameWsUpdateMsg,
    },
};

use super::{
    to::{ChessMove, Column, Position, Row},
    PieceMoveState, SelectedPiece,
};

pub(super) fn handle_pawn_click(
    mut event: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    player_color: Res<PlayerColorResource>,
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
            if player_color.0 != chess_piece.color {
                return false;
            };
            true
        }) else {
            continue;
        };
        debug!("Clicked on chess piece {:?}", clicked_entity);
        selected_piece.0 = Some(clicked_entity);
        next_state.set(PieceMoveState::PieceSelected);
    }
}

pub(super) fn handle_field_click(
    mut ev: EventReader<Pointer<Click>>,
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
        debug!("Clicked on tile {:?}!", queried_tile);
        if !queried_piece
            .1
            .is_move_valid(queried_tile.1, &piece_vec[..])
        {
            debug!(
                "Invalid move from {:?} to {:?}",
                queried_piece, queried_tile.1
            );
            continue;
        }
        // Send the turn data to the server, naively assume it's a correct move
        let get_coord = |coord: i32| match queried_piece.1.color {
            ChessPieceColorEnum::White => coord as u8,
            ChessPieceColorEnum::Black => 7 - (coord as u8),
        };
        let chess_move = ChessMove {
            position_from: Position {
                column: Column(get_coord(queried_piece.1.x)),
                row: Row(get_coord(queried_piece.1.y)),
            },
            position_to: Position {
                column: Column(get_coord(queried_tile.1.x)),
                row: Row(get_coord(queried_tile.1.y)),
            },
        };
        debug!("Moving the piece");
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
        debug!(
            "Moved the chess piece {:?} to {:?}",
            queried_piece.1, queried_tile.1
        );
        queried_piece.0.translation.x = queried_tile.0.translation.x;
        queried_piece.0.translation.y = queried_tile.0.translation.y;
        queried_piece.1.x = queried_tile.1.x;
        queried_piece.1.y = queried_tile.1.y;
        // Stop the turn
    }
}

pub(crate) fn ws_get_turn(
    mut next_move_state: ResMut<NextState<PieceMoveState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    ws_update: Res<WsUpdate>,
) {
    let Some(GameWsUpdateMsg::Game(GameMessage::NewTurn(is_turn))) = ws_update.0 else {
        return;
    };
    info!("Setting up turn state: {:?}", is_turn);
    if is_turn {
        next_move_state.set(PieceMoveState::TurnBeginning);
        next_turn_state.set(TurnState::PlayersTurn);
    } else {
        next_move_state.set(PieceMoveState::TurnBeginning);
        next_turn_state.set(TurnState::WaitingTurn);
    }
}

pub(crate) fn ws_get_move(
    player_color: Res<PlayerColorResource>,
    mut q_pieces: Query<(&mut Transform, &mut ChessPiece)>,
    mut ws_update: ResMut<WsUpdate>,
) {
    let Some(GameWsUpdateMsg::Game(GameMessage::PawnMove(ChessMove {
        ref mut position_from,
        ref mut position_to,
    }))) = ws_update.0
    else {
        return;
    };
    if let ChessPieceColorEnum::Black = player_color.0 {
        position_from.row.0 = 7 - position_from.row.0;
        position_from.column.0 = 7 - position_from.column.0;
        position_to.row.0 = 7 - position_to.row.0;
        position_to.column.0 = 7 - position_to.column.0;
    };
    debug!("Moving piece from {:?}", position_from);
    let Some((mut transform, mut chess_piece_component)) =
        q_pieces.iter_mut().find(|(_, chess_piece_component)| {
            chess_piece_component.x as u8 == position_from.column.0
                && chess_piece_component.y as u8 == position_from.row.0
        })
    else {
        error!("Not found a piece in location {:?}", position_from);
        return;
    };
    debug!("Moving piece to {:?}", position_to);
    transform.translation.x = position_to.column.0 as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    transform.translation.y = position_to.row.0 as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    chess_piece_component.x = position_to.column.0 as i32;
    chess_piece_component.y = position_to.row.0 as i32;
}
