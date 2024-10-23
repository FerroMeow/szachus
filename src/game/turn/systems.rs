use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::{
    chessboard::{
        components::{ChessBoardTile, ChessPiece, ChessPieceAlive, ChessPieceColor, MainCamera},
        systems::{BOARD_SIZE, TILE_SIZE},
    },
    ChessPieceColorEnum,
};

pub(crate) fn handle_click(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_chess_pieces: Query<(&ChessPieceAlive, &ChessPieceColor, &Transform), With<ChessPiece>>,
    q_tiles: Query<&ChessBoardTile>,
) {
    let cursor = q_window.single().cursor_position().unwrap();
    let (camera, camera_transform) = q_camera.single();
    let Vec2 { x, y } = camera
        .viewport_to_world_2d(camera_transform, cursor)
        .unwrap();
    let Some(active_piece) = q_chess_pieces.iter().find(|(alive, color, transform)| {
        let true = alive.0 else {
            return false;
        };
        let ChessPieceColorEnum::White = color.0 else {
            return false;
        };
        let Vec2 { x: x_pos, y: y_pos } = transform.translation.xy();
        if x_pos - TILE_SIZE * 0.5 <= x
            && x <= x_pos + TILE_SIZE * 0.5
            && y_pos - TILE_SIZE * 0.5 <= y
            && y <= y_pos + TILE_SIZE * 0.5
        {
            return true;
        }
        false
    }) else {
        return;
    };
    debug!("Got the chess piece");
}
