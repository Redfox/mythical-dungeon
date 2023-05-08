use bevy::prelude::*;
use systems::layout::{spawn_ui, despawn_battle_action};
use crate::game::GameState;

mod systems;
mod components;
mod styles;


pub struct BattleUIPlugin;

impl Plugin for BattleUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_ui.in_schedule(OnEnter(GameState::Battle)))
            .add_system(despawn_battle_action.in_schedule(OnExit(GameState::Battle)));
    }
}