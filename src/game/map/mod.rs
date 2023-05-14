use bevy::prelude::*;
use systems::{spawn_map, handle_events, despawn_map};
use crate::AppState;
use crate::game::GameState;

mod components;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_map.in_schedule(OnEnter(GameState::Map)))
            .add_system(handle_events.in_set(OnUpdate(GameState::Map)))
            .add_system(despawn_map.in_schedule(OnExit(GameState::Map)));
    }
}
