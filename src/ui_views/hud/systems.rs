use crate::game::{
    chessboard::systems::{BOARD_SIZE, TILE_SIZE},
    resources::PlayerColorResource,
    ChessPieceColorEnum, TurnState,
};

use super::{components::*, GameTimeElapsed};
use bevy::prelude::*;

fn spawn_player_color_text(color: ChessPieceColorEnum) -> impl Bundle {
    TextBundle::from_section(
        match color {
            ChessPieceColorEnum::Black => "You're black.",
            ChessPieceColorEnum::White => "You're white.",
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

fn spawn_time_text() -> impl Bundle {
    (
        TimeTextComponent,
        TextBundle::from_sections([
            TextSection::new("Elapsed:\n", TextStyle::default()),
            TextSection::new("00", TextStyle::default()),
            TextSection::new(":", TextStyle::default()),
            TextSection::new("00", TextStyle::default()),
            TextSection::new(":", TextStyle::default()),
            TextSection::new("00", TextStyle::default()),
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
                    width: Val::Px(TILE_SIZE * BOARD_SIZE as f32 * 0.25),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(spawn_player_color_text(r_player_color.0));
            parent.spawn(spawn_player_turn_text(*s_player_turn.get()));
            parent.spawn(spawn_time_text());
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

pub fn update_time_elapsed(
    mut q_turn_text: Query<&mut Text, With<TimeTextComponent>>,
    mut r_time_elapsed: ResMut<GameTimeElapsed>,
) {
    let time_elapsed = r_time_elapsed.0;
    let hours = time_elapsed / 3600;
    let minutes = (time_elapsed / 60) % 60;
    let seconds = time_elapsed % 60;
    for mut text in &mut q_turn_text {
        text.sections[1].value = format!("{hours:02}");
        text.sections[3].value = format!("{minutes:02}");
        text.sections[5].value = format!("{seconds:02}");
    }
    r_time_elapsed.0 = time_elapsed + 1;
}

pub fn despawn_hud(mut commands: Commands, q_hud: Query<Entity, With<HudComponent>>) {
    let single_result = q_hud.get_single();
    if let Ok(hud) = single_result {
        commands.entity(hud).despawn_recursive();
    }
}
