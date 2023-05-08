use bevy::core::Name;
use bevy::prelude::*;
use bevy::prelude::Interaction;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::*;
use crate::game::GameState;

#[derive(Component)]
pub struct Point {}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..5 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform {
                    translation: Vec3::new(i as f32 * 100. / 2. + 100., 500., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Point {},
            PickableBundle::default(),
            Name::new("Point")
        ));
    }
}

pub fn print_events(
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

pub fn despawn_map(mut commands: Commands, point_query: Query<Entity, With<Point>>) {
    for point_entity in point_query.iter() {
        commands.entity(point_entity).despawn_recursive();
    }
}
