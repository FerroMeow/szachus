use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const BOARD_SIZE: usize = 8;
const TILE_SIZE: f32 = 100.0;

pub fn create_camera(mut commands: Commands) {
    let middle_point = BOARD_SIZE as f32 * TILE_SIZE / 2.0;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(middle_point, middle_point, 0.0),
        ..default()
    });
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
            commands.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(tile.clone()),
                material: colors[(x_pos + y_pos) % 2].clone(),
                transform: Transform::from_xyz(
                    x_pos as f32 * TILE_SIZE,
                    y_pos as f32 * TILE_SIZE,
                    0.0,
                ),
                ..default()
            });
        }
    }
}

pub fn add_chess_pieces(commands: Commands) {}
