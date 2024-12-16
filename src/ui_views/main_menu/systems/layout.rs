use bevy::prelude::*;

use super::super::components::{MainMenuComponent, MainMenuStartButton};

pub fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::linear_rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            MainMenuComponent,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::vertical(Val::Px(8.0))
                                .with_left(Val::Px(16.0))
                                .with_right(Val::Px(16.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    MainMenuStartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start game",
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenuComponent>>) {
    commands
        .entity(query.get_single().unwrap())
        .despawn_recursive();
}
