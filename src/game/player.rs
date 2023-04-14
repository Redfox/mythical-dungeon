use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;
use crate::game::components::{Player, Velocity};

pub struct PlayerPlugin;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .register_ldtk_entity::<Player>("Player")
            .add_system(player)
            .add_system(animate_sprite);
    }
}

fn player(mut players: Query<(&mut Player, &mut Transform, &mut Velocity)>) {
    // for (mut player, mut transform, mut velocity) in players.iter_mut() {
    //     println!("z index: {}", transform.translation.z);
    //     transform.translation.z += 1.;
    // }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let left = -1000. / 2.;
    let padding_left = 30.;

    let mut group = Vec::new();
    group.push(Player {
        name: "Medusa".to_string(),
        speed: 1.,
        color: Color::BLUE,
        skills: vec!["Fireball".to_string()],
        size: 50.,
        texture: asset_server.load("1x/Bog Dwellers/Medusa_idle.png"),
    });

    group.push(Player {
        name: "Rogue".to_string(),
        speed: 2.,
        color: Color::GREEN,
        skills: vec!["Stealth".to_string()],
        size: 50.,
        texture: asset_server.load("2x/Bog Dwellers/Slime_idle.png"),
    });

    for (i, player) in group.iter().enumerate() {
        let texture_atlas =
            TextureAtlas::from_grid(player.texture.clone(), Vec2::new(126. / 4., 126. / 4.), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 8, last: 11 };

        commands
            .spawn((
                SpriteSheetBundle {
                    transform: Transform::from_xyz(-20. * i as f32, -20., 50.),
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Player { name: player.name.clone(), speed: player.speed, color: player.color, skills: player.skills.clone(), size: player.size, texture: player.texture.clone() },
            ));
    }
}