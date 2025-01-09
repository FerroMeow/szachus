use async_channel::Sender;
use bevy::{prelude::*, tasks::IoTaskPool, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

use crate::{
    game::{
        chessboard::{
            components::{ChessBoardTile, ChessPiece},
            systems::{SPRITE_SIZE, TILE_SIZE},
        },
        resources::{GameWinner, PlayerColorResource},
        turn::components::StartTile,
        TurnState,
    },
    network::{
        resources::{WebsocketChannels, WsUpdate},
        GameClientMsg, GameServerMsg, MatchmakingServerMsg, ServerMsg,
    },
};

use super::{
    to::{ChessMove, Position},
    GameState, PieceMoveState, RemovedPieceCount, SelectedPiece,
};

type WithChessPiece = (With<ChessPiece>, Without<ChessBoardTile>);

#[allow(clippy::too_many_arguments)]
pub(super) fn on_select_target(
    mut commands: Commands,
    mut event: EventReader<Pointer<Click>>,
    mut next_state: ResMut<NextState<PieceMoveState>>,
    player_color: Res<PlayerColorResource>,
    q_pieces: Query<(Entity, &ChessPiece), WithChessPiece>,
    q_tiles: Query<&ChessBoardTile, (With<ChessBoardTile>, Without<StartTile>)>,
    q_start_tile: Query<Entity, With<StartTile>>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for event in event.read() {
        let PointerButton::Primary = event.button else {
            continue;
        };
        let selected_entity = event.target;
        let Ok(selected_tile) = q_tiles.get(selected_entity) else {
            continue;
        };
        let Some((selected_entity, _)) = q_pieces.iter().find(|(_, chess_piece)| {
            if !chess_piece.alive {
                return false;
            }
            if player_color.0 != chess_piece.color {
                return false;
            };
            if chess_piece.x != selected_tile.x || chess_piece.y != selected_tile.y {
                return false;
            }
            true
        }) else {
            continue;
        };
        for entity in q_start_tile.iter() {
            commands.entity(entity).remove::<StartTile>();
        }
        commands.entity(selected_entity).insert(StartTile);
        selected_piece.0 = Some(selected_entity);
        next_state.set(PieceMoveState::PieceSelected);
        break;
    }
}

async fn ws_send_turn_end(tx: Sender<GameClientMsg>, chess_move: ChessMove) {
    tx.send(GameClientMsg::TurnEnd(chess_move)).await.unwrap()
}

pub(super) fn on_select_destination(
    mut commands: Commands,
    mut ev: EventReader<Pointer<Click>>,
    mut q_pieces: Query<&mut ChessPiece, Without<ChessBoardTile>>,
    q_tiles: Query<&ChessBoardTile, (Without<ChessPiece>, Without<StartTile>)>,
    q_start_tile: Query<Entity, With<StartTile>>,
    selected_piece: Res<SelectedPiece>,
    websocket_channels: Res<WebsocketChannels>,
) {
    let Some(selected_piece) = selected_piece.0 else {
        return;
    };
    for ev in ev.read() {
        let Some(tile) = q_tiles.get(ev.target).ok() else {
            continue;
        };
        let piece_vec = q_pieces.iter().copied().collect::<Vec<_>>();
        let piece = q_pieces.get_mut(selected_piece).unwrap();
        if !piece.is_move_valid(tile, &piece_vec[..]) {
            continue;
        }
        // Send the turn data to the server, naively assume it's a correct move
        IoTaskPool::get()
            .spawn(ws_send_turn_end(
                websocket_channels.tx_control.clone(),
                ChessMove {
                    position_from: Position::new(piece.x, piece.y),
                    position_to: Position::new(tile.x, tile.y),
                },
            ))
            .detach();
        // Stop the turn
        for start_tile in q_start_tile.iter() {
            commands.entity(start_tile).remove::<StartTile>();
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
    next_move_state.set(PieceMoveState::TurnBeginning);
    next_turn_state.set(if is_turn {
        TurnState::PlayersTurn
    } else {
        TurnState::WaitingTurn
    });
}

pub(crate) fn ws_get_move(
    mut commands: Commands,
    mut q_pieces: Query<(Entity, &mut Transform, &mut ChessPiece)>,
    r_player_color: Res<PlayerColorResource>,
    mut r_removed_piece_count: ResMut<RemovedPieceCount>,
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
        let removed_piece = q_pieces.iter_mut().find(|(_, _, piece)| {
            piece.color == *color
                && piece.x as i8 == position.column
                && piece.y as i8 == position.row
        });
        if let Some(mut removed_piece) = removed_piece {
            commands
                .entity(removed_piece.0)
                .remove::<(ChessPiece, PickableBundle)>();
            if removed_piece.2.color == r_player_color.0 {
                commands.entity(removed_piece.0).despawn();
            } else {
                let removed_count = r_removed_piece_count.0;
                removed_piece.1.translation.x =
                    TILE_SIZE * 8.5 + TILE_SIZE * (removed_count % 2) as f32;
                removed_piece.1.translation.y =
                    TILE_SIZE * 0.5 + TILE_SIZE * (removed_count / 2) as f32;
                r_removed_piece_count.0 += 1;
            }
        };
    };
    let Some((_, mut transform, mut chess_piece_component)) =
        q_pieces.iter_mut().find(|(_, _, chess_piece_component)| {
            chess_piece_component.x as i8 == position_from.column
                && chess_piece_component.y as i8 == position_from.row
        })
    else {
        return;
    };
    transform.translation.x = position_to.column as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    transform.translation.y = position_to.row as f32 * TILE_SIZE + TILE_SIZE * 0.5;
    chess_piece_component.x = position_to.column as i32;
    chess_piece_component.y = position_to.row as i32;
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

pub fn ws_get_error(mut s_game_state: ResMut<NextState<GameState>>, r_ws_update: Res<WsUpdate>) {
    let Some(ServerMsg::Matchmaking(ref matchmaking_server_message)) = r_ws_update.0 else {
        return;
    };
    match matchmaking_server_message {
        MatchmakingServerMsg::Error(_) => (),
        MatchmakingServerMsg::GameDropped(_) => (),
        _ => {
            return;
        }
    };
    s_game_state.set(GameState::Error);
}
