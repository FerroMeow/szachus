use bevy::{color::palettes::css::WHITE, prelude::*};

pub fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(Vec2::ZERO, 0.0, UVec2::splat(300), Vec2::splat(100.), WHITE)
        .outer_edges();
}
