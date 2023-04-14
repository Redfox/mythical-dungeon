use bevy::prelude::*;
use bevy::prelude::Component;
use bevy::sprite::MaterialMesh2dBundle;
use crate::game::components::Player;
use crate::game::WinSize;

pub struct BattlePlugin;

#[derive(Component, Debug)]
pub struct PlayerBattle {
    pub position: f32,
    pub speed: f32,
    pub size: f32,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui_battle.in_base_set(StartupSet::PostStartup))
            .add_startup_system(spawn_player_battle.in_base_set(StartupSet::PostStartup))
            .add_system(handle_battle)
            .add_system(handle_mouse_hover);
    }
}

fn setup_ui_battle(
    mut commands: Commands,
    mut players: Query<(&mut Player)>
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(600.),
                            height: Val::Px(40.),
                        },
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(25.),
                            ..default()
                        },
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Px(100.),
                                    height: Val::Px(40.),
                                    ..default()
                                },
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            background_color: Color::RED.into(),
                            ..default()
                        });

                    for player in players.iter_mut() {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Px(player.size),
                                        height: Val::Px(player.size),
                                        ..default()
                                    },
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(0.),
                                        top: Val::Px(0. - player.size),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: player.color.into(),
                                ..default()
                            })
                            .insert(PlayerBattle { speed: player.speed, position: 0., size: player.size });
                    }
                });
        });
}

fn spawn_player_battle(
    mut commands: Commands,
    mut players: Query<(&mut Player)>,
    win_size: Res<WinSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for player in players.iter_mut() {
        let player_circle = -win_size.width / 2. + 200. + (10. * 2.) + 5. + 3.;
        // commands
        //     .spawn(MaterialMesh2dBundle {
        //         mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        //         material: materials.add(ColorMaterial::from(player.color)),
        //         transform: Transform {
        //             translation: Vec3::new(-600. / 2., player_circle, 0.),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     })
        //     .insert(PlayerBattle { speed: player.speed });
    };
}

fn handle_battle(
    mut query: Query<(&mut Style, &mut PlayerBattle), With<PlayerBattle>>
) {
    for (mut style, mut player) in query.iter_mut() {
        // let lol = style.size.height as f32;
        //
        // if player.position - style.size.height as f32 < 600. {
        //     player.position += player.speed;
        //     style.position.left = Val::Px(player.position);
        // }

        if player.position < 600. - player.size {
            player.position += player.speed;
            style.position.left = Val::Px(player.position);
        }
    }
}

fn handle_mouse_hover(
    mut windows: Query<&mut Window>,
    mut players: Query<(&mut Transform, &mut Player, &mut Sprite), With<Player>>
) {
    for (transform, player, mut sprite) in players.iter_mut() {
        let middle_screen_x = 1000. / 2.;
        let middle_screen_y = 700. / 2.;
        let init_pos_x = transform.translation.x + middle_screen_x;
        let init_pos_y = transform.translation.y + middle_screen_y;

        let window = windows.single_mut();

        let position = window.cursor_position();

        match position {
            Some(position) => {
                if
                    position.x > init_pos_x - (70. / 2.) && position.x < init_pos_x + (70. / 2.)
                    && position.y > init_pos_y - (70. / 2.) && position.y < init_pos_y + (70. / 2.)
                {
                    // sprite.color = Color::RED;
                } else {
                    // sprite.color = player.color;
                }
            },
            None => {
                // sprite.color = player.color;
            }
        }
    }
}