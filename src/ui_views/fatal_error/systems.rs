use bevy::prelude::*;

use crate::ui_views::retry_game::RetryBtn;

use super::components::FatalErrorScreenComponent;

pub fn spawn(mut commands: Commands) {
    // Screen
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
        background_color: Color::linear_rgb(255.0, 0.0, 0.0).into(),
        ..default()
    };

    // Win text
    let win_text_style = TextStyle {
        color: Color::BLACK,
        ..default()
    };
    let win_text_node = TextBundle::from_section(
        "The game encountered a critical error. Please try playing again!",
        win_text_style.clone(),
    );

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
        .spawn((screen_node, FatalErrorScreenComponent))
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
    q_game_over_screen: Query<Entity, With<FatalErrorScreenComponent>>,
) {
    if let Ok(entity) = q_game_over_screen.get_single() {
        commands.entity(entity).despawn_recursive();
    };
}
