pub mod resources;
mod system;

use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy::system::enemy_resource;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App)  {
        app
            .add_system(
              enemy_resource.in_schedule(OnEnter(AppState::Game))
            );
    }
}