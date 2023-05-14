mod system;
pub mod resources;

use bevy::prelude::*;
use crate::AppState;
use crate::game::GameState;
use crate::game::player::system::{player_resource, player_attack};
use bevy_inspector_egui::quick::{WorldInspectorPlugin, ResourceInspectorPlugin};
use crate::game::player::resources::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App)  {
        app
            .add_system(
                player_resource.in_schedule(OnEnter(AppState::Game))
            )
            .add_system(
                player_attack
                    .in_set(OnUpdate(GameState::Battle))
            );

    }
}