use bevy::core::Name;
use bevy::prelude::*;
use bevy::prelude::Interaction;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::*;
use crate::game::GameState;

#[derive(Component)]
pub struct Point {}

#[derive(Component)]
pub struct Line {}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let positions = [100., 200., 300., 400., 500.];

    for i in 0..5 {

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform {
                    translation: Vec3::new(positions[i], window.height() / 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Point { },
            PickableBundle::default(),
            Name::new("Point")
        ));

        if i > 0 {
            let size = positions[i - 1] - positions[i];
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0., 0., 0., 0.2),
                        custom_size: Some(Vec2::new(size, 4.)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(positions[i] + (size / 2.), window.height() / 2., 0.)),
                    ..default()
                },
                Line { },
                Name::new(format!("Line {i}"))
            ));
        }
    }
}

pub fn handle_events(
    mut events: EventReader<PickingEvent>,
    mut app_state_next_state: ResMut<NextState<GameState>>
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                app_state_next_state.set(GameState::Battle);
            },

            _ => {}
        }
    }
}

pub fn despawn_map(
    mut commands: Commands,
    point_query: Query<Entity, With<Point>>,
    line_query: Query<Entity, With<Line>>
) {
    for entities in point_query.iter().chain(line_query.iter()) {
        commands.entity(entities).despawn_recursive();
    }
}
