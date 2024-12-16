use core::panic;

use super::{components::*, ChessPieceTypeEnum};
use crate::game::{resources::PlayerColorResource, ChessPieceColorEnum};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;

pub(crate) const BOARD_SIZE: i32 = 8;
pub(crate) const TILE_SIZE: f32 = 32.0;

pub fn create_camera(mut commands: Commands) {
    let middle_point = BOARD_SIZE as f32 * TILE_SIZE / 2.0;
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(middle_point, middle_point, 0.0),
            ..default()
        },
        MainCamera,
    ));
}

pub fn draw_chessboard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    let colors = [Color::BLACK, Color::WHITE].map(|color| materials.add(color));

    for y_pos in 0..BOARD_SIZE {
        for x_pos in 0..BOARD_SIZE {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(tile.clone()),
                    material: colors[((x_pos + y_pos) as usize) % 2].clone(),
                    transform: Transform::from_xyz(
                        x_pos as f32 * TILE_SIZE + TILE_SIZE * 0.5,
                        y_pos as f32 * TILE_SIZE + TILE_SIZE * 0.5,
                        0.0,
                    ),
                    ..default()
                },
                ChessBoardTile { x: x_pos, y: y_pos },
                PickableBundle::default(),
            ));
        }
    }
}

pub fn add_chess_pieces(
    mut commands: Commands,
    player_color: Res<PlayerColorResource>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout_asset: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_color = player_color.0;
    let enemy_color = match player_color {
        ChessPieceColorEnum::White => ChessPieceColorEnum::Black,
        ChessPieceColorEnum::Black => ChessPieceColorEnum::White,
    };
    // Load sprites
    let chesspieces_texture: Handle<Image> = asset_server.load("textures/chesspieces.png");
    let chesspieces_layout = TextureAtlasLayout::from_grid(UVec2::new(18, 28), 6, 2, None, None);
    let chesspieces_layout_handle = texture_atlas_layout_asset.add(chesspieces_layout);
    // Spawn pawn rows
    for i in 0..8 {
        spawn_chess_piece(
            &mut commands,
            chesspieces_texture.clone(),
            chesspieces_layout_handle.clone(),
            player_color,
            ChessPieceTypeEnum::Pawn,
            IVec2::new(i, 1),
        );
        spawn_chess_piece(
            &mut commands,
            chesspieces_texture.clone(),
            chesspieces_layout_handle.clone(),
            enemy_color,
            ChessPieceTypeEnum::Pawn,
            IVec2::new(i, 6),
        );
    }
    // Spawn own first row
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Rook,
        IVec2::new(0, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Rook,
        IVec2::new(7, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Knight,
        IVec2::new(1, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Knight,
        IVec2::new(6, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Bishop,
        IVec2::new(2, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Bishop,
        IVec2::new(5, 0),
    );
    // Spawn enemy first row
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Rook,
        IVec2::new(0, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Rook,
        IVec2::new(7, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Knight,
        IVec2::new(1, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Knight,
        IVec2::new(6, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Bishop,
        IVec2::new(2, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Bishop,
        IVec2::new(5, 7),
    );
    // Spawn kings and queens
    let (queen_pos, king_pos) = match (player_color, enemy_color) {
        (ChessPieceColorEnum::White, ChessPieceColorEnum::Black) => (3, 4),
        (ChessPieceColorEnum::Black, ChessPieceColorEnum::White) => (4, 3),
        _ => panic!(),
    };
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::King,
        IVec2::new(king_pos, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        player_color,
        ChessPieceTypeEnum::Queen,
        IVec2::new(queen_pos, 0),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::King,
        IVec2::new(king_pos, 7),
    );
    spawn_chess_piece(
        &mut commands,
        chesspieces_texture.clone(),
        chesspieces_layout_handle.clone(),
        enemy_color,
        ChessPieceTypeEnum::Queen,
        IVec2::new(queen_pos, 7),
    );
}

fn spawn_chess_piece(
    commands: &mut Commands,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    color: ChessPieceColorEnum,
    chess_type: ChessPieceTypeEnum,
    tile: IVec2,
) {
    let texture_x = match chess_type {
        ChessPieceTypeEnum::Pawn => 0,
        ChessPieceTypeEnum::Knight => 1,
        ChessPieceTypeEnum::Bishop => 2,
        ChessPieceTypeEnum::Rook => 3,
        ChessPieceTypeEnum::Queen => 4,
        ChessPieceTypeEnum::King => 5,
    };
    let texture_y = match color {
        ChessPieceColorEnum::White => 0,
        ChessPieceColorEnum::Black => 6,
    };
    commands.spawn((
        ChessPiece {
            x: tile.x,
            y: tile.y,
            piece_type: chess_type,
            color,
            alive: true,
        },
        SpriteBundle {
            transform: Transform::from_xyz(
                tile.x as f32 * TILE_SIZE + TILE_SIZE * 0.5,
                tile.y as f32 * TILE_SIZE + TILE_SIZE * 0.5,
                1.0,
            ),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: texture_x + texture_y,
        },
        PickableBundle::default(),
    ));
}

pub fn clean_chessboard(
    mut commands: Commands,
    q_pieces: Query<Entity, (With<ChessPiece>, Without<ChessBoardTile>)>,
    q_tiles: Query<Entity, (With<ChessBoardTile>, Without<ChessPiece>)>,
) {
    for entity in q_pieces.iter().chain(q_tiles.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}
