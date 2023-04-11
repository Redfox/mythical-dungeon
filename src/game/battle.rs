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
            .add_system(handle_battle);
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
    if players.iter_mut().count() < 2 {
        println!("players: {:?}", players.iter_mut().count());
    }

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