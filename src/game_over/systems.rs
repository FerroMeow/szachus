use bevy::prelude::*;

use crate::game::resources::GameWinner;

use super::components::GameOverScreenComponent;

pub fn spawn_game_over_screen(mut commands: Commands, winner: Res<GameWinner>) {
    // Screen
    let screen_background = if winner.0 {
        Color::linear_rgb(0.66, 1.0, 0.66)
    } else {
        Color::linear_rgb(1.0, 0.66, 0.66)
    };
    let screen_style = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),

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
    let retry_button_style = Style { ..default() };
    let retry_button_node = ButtonBundle {
        style: retry_button_style,
        ..default()
    };

    // Start again button text
    let retry_button_text = TextBundle::from_section("Play again", win_text_style);

    commands
        .spawn((screen_node, GameOverScreenComponent))
        .with_children(|parent| {
            parent.spawn(win_text_node);
            parent
                .spawn(retry_button_node)
                .with_children(|retry_button| {
                    retry_button.spawn(retry_button_text);
                });
        });
}
