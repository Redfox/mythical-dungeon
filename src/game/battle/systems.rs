use bevy::core::Name;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use crate::game::battle::BattleEvent;
use crate::game::player::resources::{Player, Character};
use crate::game::enemy::resources::Enemy;
use bevy_inspector_egui::prelude::*;
use crate::game::GameState;
use crate::game::battle::components::*;

pub fn startup(mut app_state_next_state: ResMut<NextState<BattleEvent>>,) {
    app_state_next_state.set(BattleEvent::PlayerWarmup);
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Res<Player>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (i, character) in player.characters.iter().enumerate() {
        let texture = asset_server.load("1x/Bog Dwellers/Medusa_idle.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture, Vec2::new(128. / 4., 128. / 4.), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 8, last: 11 };

        commands
            .spawn((
                SpriteSheetBundle {
                    transform: Transform::from_xyz(150., 400. + (i as f32 * 100.), 0.),
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    ..default()
                },
                CharacterComponent {
                     data: character.clone()
                },
                Name::new(character.name.clone())
            ));
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy: Res<Enemy>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("1x/Bog Dwellers/Beholder_idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture, Vec2::new(128. / 4., 128. / 4.), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 4, last: 11 };

    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform::from_xyz(1000., 400., 0.),
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                ..default()
            },
            EnemyComponent {
                max_hp: enemy.health,
                hp: enemy.health,
            },
            Name::new(enemy.name.clone())
        ))
        .with_children(| parent | {
            parent
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::new(100., 10.)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::BLACK)),
                        transform: Transform::from_xyz(0., -25., -1.),
                        ..default()
                    },
                    HealthBar { },
                    Name::new("Enemy Health Bar")
                ))
                .with_children(| parent | {
                    let max_hp = enemy.health;
                    let enemy_hp = enemy.health;

                    let percent = enemy_hp / max_hp;

                    let min: f32 = -50.;
                    let max = 0.;

                    let result = (max - min) as f32 * percent;
                    let final_result = result - min.abs();

                    parent
                        .spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(shape::Quad::new(Vec2::new(100. * percent, 10.)).into())
                                    .into(),
                                material: materials.add(ColorMaterial::from(Color::GREEN)),
                                transform: Transform::from_xyz(final_result, 0., 1.),
                                ..default()
                            },
                            HealthBarFill { },
                            Name::new("Enemy Health Bar Fill")
                        ));
                });
        });
}

pub fn char_skills(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    characters: Query<(&Transform, &CharacterComponent)>,
) {
    for (transform, char) in characters.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(10., 10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_xyz(transform.translation.x + 30., transform.translation.y - 5., transform.translation.z),
                ..default()
            },
            Name::new("Skill"),
            CharSkill { }
        ));
    }
}

pub fn char_attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    characters: Query<(&Transform, &CharacterComponent)>,
    enemy_query: Query<Entity, (With<EnemyComponent>, Without<CharSkill>)>,
) {
    for (i, (transform, char)) in characters.iter().enumerate() {
        if i > 0 || enemy_query.iter().count() <= 0 {
            continue;
        }

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(10., 10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(transform.translation.x + 5., transform.translation.y - 5., transform.translation.z),
                ..default()
            },
            Skill { }
        ));
    }
}

pub fn char_skill_animation(
    mut commands: Commands,
    mut skill_query: Query<(&mut Transform, Entity), With<Skill>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyComponent, Entity), (With<EnemyComponent>, Without<Skill>)>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
) {
    for (mut skill_transform, skill_entity) in skill_query.iter_mut() {
        if enemy_query.iter().count() <= 0 {
            commands.entity(skill_entity).despawn();
        }

        if let Ok((enemy_transform, mut enemy, enemy_entity)) = enemy_query.get_single_mut() {
            if skill_transform.translation.x < enemy_transform.translation.x {
                skill_transform.translation.x += 10.;
            }

            if skill_transform.translation.y < enemy_transform.translation.y {
                skill_transform.translation.y += 10.;
            }

            if skill_transform.translation.x - 5. == enemy_transform.translation.x && skill_transform.translation.y - 5. == enemy_transform.translation.y {
                enemy.hp -= 10.;
                if enemy.hp <= 0. {
                    enemy.hp = 0.;

                    commands.entity(enemy_entity).despawn();
                    app_state_next_state.set(GameState::Map);
                }

                commands.entity(skill_entity).despawn();
            }
        }
    }
}

pub fn despawn_system(
    mut commands: Commands,
    char_skills_entities: Query<Entity, With<CharSkill>>,
    skills_entities: Query<Entity, With<Skill>>,
    enemy_entities: Query<Entity, With<EnemyComponent>>,
    char_entities: Query<Entity, With<CharacterComponent>>,
    health_bar_entities: Query<Entity, With<HealthBar>>,
    health_bar_fill_entities: Query<Entity, With<HealthBarFill>>,
) {
    let entities = char_skills_entities.iter()
                                        .chain(skills_entities.iter())
                                        .chain(enemy_entities.iter())
                                        .chain(char_entities.iter())
                                        .chain(health_bar_entities.iter()
                                        .chain(health_bar_fill_entities.iter()))
                                        .collect::<Vec<Entity>>();

    for entity in entities {
        commands.entity(entity).despawn();
    }
}

pub fn enemy_health_bar(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<(&EnemyComponent, &Children), (Changed<EnemyComponent>)>,
    health_bar_query: Query<(&HealthBar, &Children)>,
    mut health_bar_fill_query: Query<(&mut Transform, &mut Mesh2dHandle,  &mut Handle<ColorMaterial>, &HealthBarFill)>,
) {
    if let Ok((enemy, children)) = enemy_query.get_single() {
        for &child in children.iter() {
            if let Ok((health_bar, health_bar_children)) = health_bar_query.get(child) {
                for &health_bar_child in health_bar_children.iter() {
                    if let Ok((mut health_bar_transform, mut mesh, color, _)) = health_bar_fill_query.get_mut(health_bar_child) {
                        let max_hp = enemy.max_hp;
                        let enemy_hp = enemy.hp;

                        let percent = enemy_hp / max_hp;

                        let min: f32 = -50.;
                        let max = 0.;

                        let result = (max - min) as f32 * percent;
                        let final_result = result - min.abs();

                        mesh.0 = meshes
                            .add(shape::Quad::new(Vec2::new(100. * percent, 10.)).into())
                            .into();

                        if percent <= 0.25 {
                            let mut color_mat = materials.get_mut(&color).unwrap();
                            color_mat.color = Color::RED;
                        }

                        health_bar_transform.translation.x = final_result;
                    }
                }
            }
        }
    }
}