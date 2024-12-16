use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_mod_picking::prelude::*;

use crate::{
    game::{
        chessboard::{
            components::{ChessBoardTile, ChessPiece},
            systems::TILE_SIZE,
        },
        resources::{GameWinner, PlayerColorResource},
        ChessPieceColorEnum, TurnState,
    },
    network::{
        resources::{WebsocketChannels, WsUpdate},
        GameClientMsg, GameServerMsg, ServerMsg,
    },
};

use super::{
    to::{ChessMove, Position},
    GameState, PieceMoveState, SelectedPiece,
};

pub(super) fn handle_pawn_click(
    mut commands: Commands,
    mut event: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    player_color: Res<PlayerColorResource>,
    q_pieces: Query<(Entity, &ChessPiece), With<ChessPiece>>,
    q_tiles: Query<Entity, (With<ChessBoardTile>, Without<ChessPiece>)>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for event in event.read() {
        let PointerButton::Primary = event.button else {
            continue;
        };
        let clicked_entity = event.target;

        let Some(_) = q_pieces.iter().find(|(entity, chess_piece)| {
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
        for piece in q_pieces.iter() {
            commands.entity(piece.0).remove::<PickableBundle>();
        }
        for tile in q_tiles.iter() {
            commands.entity(tile).insert(PickableBundle {
                pickable: Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
                ..default()
            });
        }
        next_state.set(PieceMoveState::PieceSelected);
    }
}

pub(super) fn handle_field_click(
    mut commands: Commands,
    mut ev: EventReader<Pointer<Click>>,
    mut q_pieces: Query<(Entity, &mut Transform, &mut ChessPiece), Without<ChessBoardTile>>,
    q_tiles: Query<(Entity, &Transform, &ChessBoardTile), Without<ChessPiece>>,
    selected_piece: Res<SelectedPiece>,
    websocket_channels: Res<WebsocketChannels>,
) {
    let Some(selected_piece) = selected_piece.0 else {
        return;
    };
    for ev in ev.read() {
        let Some(queried_tile) = q_tiles.get(ev.target).ok() else {
            continue;
        };
        let piece_vec = q_pieces
            .iter()
            .map(|(_, _, piece)| *piece)
            .collect::<Vec<_>>();
        let mut queried_piece = q_pieces.get_mut(selected_piece).unwrap();
        debug!("Clicked on tile {:?}!", queried_tile);
        if !queried_piece
            .2
            .is_move_valid(queried_tile.2, &piece_vec[..])
        {
            debug!(
                "Invalid move from {:?} to {:?}",
                queried_piece, queried_tile.1
            );
            continue;
        }
        // Send the turn data to the server, naively assume it's a correct move
        let get_coord = |coord: i32| match queried_piece.2.color {
            ChessPieceColorEnum::White => coord as i8,
            ChessPieceColorEnum::Black => 7 - (coord as i8),
        };
        let chess_move = ChessMove {
            position_from: Position {
                column: get_coord(queried_piece.2.x),
                row: get_coord(queried_piece.2.y),
            },
            position_to: Position {
                column: get_coord(queried_tile.2.x),
                row: get_coord(queried_tile.2.y),
            },
        };
        debug!("Moving the piece");
        let tx_current = websocket_channels.tx_control.clone();
        IoTaskPool::get()
            .spawn(async move {
                tx_current
                    .send(GameClientMsg::TurnEnd(chess_move))
                    .await
                    .unwrap()
            })
            .detach();
        // Move the piece to the desired location
        debug!(
            "Moved the chess piece {:?} to {:?}",
            queried_piece.1, queried_tile.1
        );
        queried_piece.1.translation.x = queried_tile.1.translation.x;
        queried_piece.1.translation.y = queried_tile.1.translation.y;
        queried_piece.2.x = queried_tile.2.x;
        queried_piece.2.y = queried_tile.2.y;
        // Stop the turn

        for piece in q_pieces.iter() {
            commands.entity(piece.0).insert(PickableBundle {
                pickable: Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
                ..default()
            });
        }
        for tile in q_tiles.iter() {
            commands.entity(tile.0).remove::<PickableBundle>();
        }
        break;
    }
}

pub(crate) fn ws_get_turn(
    mut next_move_state: ResMut<NextState<PieceMoveState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    ws_update: Res<WsUpdate>,
) {
    let Some(ServerMsg::Game(GameServerMsg::NewTurn(is_turn))) = ws_update.0 else {
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
    mut commands: Commands,
    player_color: Res<PlayerColorResource>,
    mut q_pieces: Query<(Entity, &mut Transform, &mut ChessPiece)>,
    mut ws_update: ResMut<WsUpdate>,
) {
    let Some(ServerMsg::Game(GameServerMsg::PawnMove(
        ChessMove {
            ref mut position_from,
            ref mut position_to,
        },
        ref removed_piece,
    ))) = ws_update.0
    else {
        return;
    };
    if let Some((color, position)) = removed_piece {
        let position = match player_color.0 {
            ChessPieceColorEnum::Black => Position {
                column: 7 - position.column,
                row: 7 - position.row,
            },
            ChessPieceColorEnum::White => Position {
                column: position.column,
                row: position.row,
            },
        };
        debug!("Searching for piece with color {color:?} and position {position:?}");
        let piece_to_remove = q_pieces.iter().find(|(_, _, piece)| {
            piece.color == *color
                && piece.x as i8 == position.column
                && piece.y as i8 == position.row
        });
        if let Some((entity_to_remove, _, _)) = piece_to_remove {
            commands.entity(entity_to_remove).despawn();
            debug!("Removed the entity at position {position:?}");
        } else {
            debug!("Not found a piece  with color {color:?} and position {position:?}");
        }
    };
    if let ChessPieceColorEnum::Black = player_color.0 {
        position_from.row = 7 - position_from.row;
        position_from.column = 7 - position_from.column;
        position_to.row = 7 - position_to.row;
        position_to.column = 7 - position_to.column;
    };
    debug!("Moving piece from {:?}", position_from);
    let Some((_, mut transform, mut chess_piece_component)) =
        q_pieces.iter_mut().find(|(_, _, chess_piece_component)| {
            chess_piece_component.x as i8 == position_from.column
                && chess_piece_component.y as i8 == position_from.row
        })
    else {
        error!("Not found a piece in location {:?}", position_from);
        return;
    };
    debug!("Moving piece to {:?}", position_to);
    transform.translation.x = position_to.column as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    transform.translation.y = position_to.row as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    chess_piece_component.x = position_to.column as i32;
    chess_piece_component.y = position_to.row as i32;
}

pub fn ws_get_confirm(
    mut commands: Commands,
    player_color: Res<PlayerColorResource>,
    q_pieces: Query<(Entity, &ChessPiece)>,
    ws_update: Res<WsUpdate>,
) {
    let Some(ServerMsg::Game(GameServerMsg::MovedCorrectly(Some((ref color, ref position))))) =
        ws_update.0
    else {
        return;
    };
    let position = match player_color.0 {
        ChessPieceColorEnum::Black => Position {
            column: 7 - position.column,
            row: 7 - position.row,
        },
        ChessPieceColorEnum::White => Position {
            column: position.column,
            row: position.row,
        },
    };
    if let Some((entity, _)) = q_pieces.iter().find(|(_, piece)| {
        piece.color == *color && piece.x as i8 == position.column && piece.y as i8 == position.row
    }) {
        commands.entity(entity).despawn()
    }
}

pub fn ws_get_win(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    websocket_channels: Res<WebsocketChannels>,
    ws_update: Res<WsUpdate>,
) {
    let Some(ServerMsg::Game(GameServerMsg::GameEnd(is_won))) = ws_update.0 else {
        return;
    };
    let tx = websocket_channels.tx_control.clone();
    IoTaskPool::get()
        .spawn(async move {
            tx.send(GameClientMsg::Close).await.unwrap();
        })
        .detach();
    commands.insert_resource(GameWinner(is_won));
    game_state.set(GameState::Finished);
}
