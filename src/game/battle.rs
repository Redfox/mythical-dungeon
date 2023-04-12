use bevy::prelude::*;
use bevy::prelude::Component;
use bevy::sprite::MaterialMesh2dBundle;
use crate::game::components::Player;
use crate::game::WinSize;

pub struct BattlePlugin;

#[derive(Component, Debug)]
pub struct PlayerBattle {
    pub speed: f32,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_battle.in_base_set(StartupSet::PostStartup))
            .add_startup_system(spawn_player_battle.in_base_set(StartupSet::PostStartup))
            .add_system(handle_battle)
            .add_system(handle_mouse_hover);
    }
}

fn setup_battle(
    mut commands: Commands,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.width / 2.;
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(600., 30.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., bottom + 200., 0.),
                    ..Default::default()
                },
                ..Default::default()
            }
        ));
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
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(player.color)),
                transform: Transform {
                    translation: Vec3::new(-600. / 2., player_circle, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerBattle { speed: player.speed });
    }
}

fn handle_battle(
    mut query: Query<(&mut Transform, &mut PlayerBattle), With<PlayerBattle>>
) {
    for (mut transform, mut player) in query.iter_mut() {
        if transform.translation.x < 300. {
            transform.translation.x += player.speed;
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
                // println!("X: {}, Y: {}", position.x, position.y);
                if
                    position.x > init_pos_x - (70. / 2.) && position.x < init_pos_x + (70. / 2.)
                    && position.y > init_pos_y - (70. / 2.) && position.y < init_pos_y + (70. / 2.)
                {
                    // println!("X no {}", player.name.clone());
                    sprite.color = Color::RED;
                } else {
                    sprite.color = player.color;
                }
            },
            None => {
                // println!("No position");
                sprite.color = player.color;
            }
        }
    }

    // let middle_screen = 500.;
    // let init_pos = transform.translation.x + middle_screen;
    //
    // let window = windows.single_mut();
    //
    // let position = window.cursor_position();
    //
    // match position {
    //     Some(position) => {
    //         // println!("X: {}, Y: {}", position.x, position.y);
    //         if position.x > init_pos && position.x < init_pos + 70. {
    //             println!("X no dale {}", position.x);
    //         }
    //     },
    //     None => {
    //         // println!("No position");
    //     }
    // }

    // println!("{:?}", window.cursor_position());
}