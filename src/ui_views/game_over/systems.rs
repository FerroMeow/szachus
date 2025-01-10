use bevy::prelude::*;

use crate::{game::resources::GameWinner, ui_views::retry_game::RetryBtn};

use super::components::GameOverScreenComponent;

pub fn spawn(mut commands: Commands, winner: Res<GameWinner>) {
    // Screen
    let screen_background = if winner.0 {
        Color::linear_rgba(0.66, 1.0, 0.66, 0.2)
    } else {
        Color::linear_rgba(1.0, 0.66, 0.66, 0.2)
    };
    let screen_style = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(16.0)),
        ..default()
    };
    let screen_node = NodeBundle {
        style: screen_style,
        background_color: screen_background.into(),
        ..default()
    };

    // Win text
    let win_text_content = if winner.0 {
        "You just won the game! Congratulations!"
    } else {
        "You lost this game! Try again."
    };
    let win_text_style = TextStyle {
        color: Color::BLACK,
        ..default()
    };
    let win_text_node = TextBundle::from_section(win_text_content, win_text_style.clone());

    // Start again button
    let retry_button_style = Style {
        padding: UiRect::horizontal(Val::Px(16.0))
            .with_top(Val::Px(8.0))
            .with_bottom(Val::Px(8.0)),
        margin: UiRect::top(Val::Px(16.0)),
        ..default()
    };
    let retry_button_node = ButtonBundle {
        style: retry_button_style,
        background_color: Color::WHITE.into(),
        border_radius: BorderRadius::all(Val::Px(8.0)),
        ..default()
    };

    // Start again button text
    let retry_button_text = TextBundle::from_section("Play again", win_text_style);

    commands
        .spawn((screen_node, GameOverScreenComponent))
        .with_children(|parent| {
            parent.spawn(win_text_node);
            parent
                .spawn((retry_button_node, RetryBtn))
                .with_children(|retry_button| {
                    retry_button.spawn(retry_button_text);
                });
        });
}

pub fn despawn(
    mut commands: Commands,
    q_game_over_screen: Query<Entity, With<GameOverScreenComponent>>,
) {
    if let Ok(entity) = q_game_over_screen.get_single() {
        commands.entity(entity).despawn_recursive();
    };
}
