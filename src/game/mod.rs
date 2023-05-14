use bevy::prelude::*;
use crate::AppState;
use crate::game::battle::BattlePlugin;
use crate::game::enemy::EnemyPlugin;
use crate::game::map::MapPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::systems::startup;
use crate::game::ui::GameUIPlugin;

mod systems;
mod map;
mod battle;
pub mod player;
mod ui;
mod enemy;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_system(startup.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(MapPlugin)
            .add_plugin(BattlePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(GameUIPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    None,
    Map,
    Battle,
}