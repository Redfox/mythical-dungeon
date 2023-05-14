use bevy::prelude::*;
use crate::main_menu::components::{MainMenu, PlayButton};
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu { },
        ))
        .with_children(|parent| {
            title(parent, &asset_server);
            play_button(parent, &asset_server);
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: TITLE_STYLE,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Mythical Dungeon",
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

fn play_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
           ButtonBundle {
               style: BUTTON_STYLE,
               background_color: NORMAL_BUTTON_COLOR.into(),
               ..default()
           },
            PlayButton { }
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
               text: Text {
                   sections: vec![TextSection::new(
                       "Play",
                       get_button_text_style(&asset_server)
                   )],
                   alignment: TextAlignment::Center,
                   ..default()
               },
                ..default()
            });
        });
}