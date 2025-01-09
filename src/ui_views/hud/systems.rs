use crate::game::{
    chessboard::systems::{BOARD_SIZE, TILE_SIZE},
    resources::PlayerColorResource,
    ChessPieceColorEnum, TurnState,
};

use super::components::*;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn spawn_player_color_text(color: ChessPieceColorEnum) -> impl Bundle {
    TextBundle::from_section(
        match color {
            ChessPieceColorEnum::Black => "You're black.\n",
            ChessPieceColorEnum::White => "You're white.\n",
        },
        TextStyle::default(),
    )
}

fn spawn_player_turn_text(turn_state: TurnState) -> impl Bundle {
    (
        TurnTextComponent,
        TextBundle::from_sections([
            TextSection::new("Move: ", TextStyle::default()),
            TextSection::new(turn_state, TextStyle::default()),
        ]),
    )
}

pub fn spawn_hud(
    mut commands: Commands,
    r_player_color: Res<PlayerColorResource>,
    s_player_turn: Res<State<TurnState>>,
) {
    commands
        .spawn((
            HudComponent,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(TILE_SIZE * BOARD_SIZE as f32 * 0.5),
                    ..default()
                },
                ..default()
            },
            PickableBundle {
                pickable: Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(spawn_player_color_text(r_player_color.0));
            parent.spawn(spawn_player_turn_text(*s_player_turn.get()));
        });
}

pub fn update_turn_text(
    mut q_turn_text: Query<&mut Text, With<TurnTextComponent>>,
    s_player_turn: Res<State<TurnState>>,
) {
    for mut text in &mut q_turn_text {
        let turn_section = TextSection::new(*s_player_turn.get(), TextStyle::default());
        text.sections[1] = turn_section;
    }
}

pub fn despawn_hud(mut commands: Commands, q_hud: Query<Entity, With<HudComponent>>) {
    let single_result = q_hud.get_single();
    if let Ok(hud) = single_result {
        commands.entity(hud).despawn_recursive();
    }
}
