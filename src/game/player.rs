use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::game::components::{Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let left = -1000. / 2.;
    let padding_left = 30.;

    let mut group = Vec::new();
    group.push(Player {
        name: "Mage".to_string(),
        speed: 1.,
        color: Color::BLUE,
        skills: vec!["Fireball".to_string()]
    });

    group.push(Player {
        name: "Rogue".to_string(),
        speed: 2.,
        color: Color::GREEN,
        skills: vec!["Stealth".to_string()]
    });

    for (i, player) in group.iter().enumerate() {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: player.color,
                        custom_size: Some(Vec2::new(70., 70.)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(left + 70. / 2. + padding_left, 200. - (200. * i as f32), 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .insert(Player { name: player.name.clone(), speed: player.speed, color: player.color, skills: player.skills.clone() })
            .insert(Velocity { x: 0., y: 0. });

        commands.spawn(
            TextBundle::from_section(
                player.name.clone(),
                TextStyle {
                    font: asset_server.load("fonts/ubuntu.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(480. - (200. * i as f32)),
                        left: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                }),
        );
    }
}