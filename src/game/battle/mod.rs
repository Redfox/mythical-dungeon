mod systems;
mod components;

use bevy::prelude::*;
use crate::game::battle::components::EnemyComponent;
use crate::game::GameState;
use crate::game::battle::systems::*;

pub struct BattlePlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum BattleEvent {
    #[default]
    Idle,
    PlayerWarmup,
    PlayerAttack,
    EnemyAttack,
    End,
}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EnemyComponent>()
            .add_state::<BattleEvent>()
            .add_systems(
                (startup, spawn_player, spawn_enemy)
                    .in_schedule(OnEnter(GameState::Battle))
            )
            .add_system(enemy_health_bar.in_set(OnUpdate(GameState::Battle)))
            .add_system(char_skills.in_schedule(OnEnter(BattleEvent::PlayerWarmup)))
            .add_system(char_attack.in_schedule(OnEnter(BattleEvent::PlayerAttack)))
            .add_system(char_skill_animation.in_set(OnUpdate(BattleEvent::PlayerAttack)))
            .add_system(despawn_system.in_schedule(OnExit(GameState::Battle)));
    }
}