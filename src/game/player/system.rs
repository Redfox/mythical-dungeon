use bevy::prelude::*;
use crate::game::battle::BattleEvent;
use crate::game::player::resources::Player;

pub fn player_resource(mut commands: Commands) {
    commands.insert_resource(Player::default());
}

pub fn player_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state_next_state: ResMut<NextState<BattleEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        app_state_next_state.set(BattleEvent::PlayerAttack);
    }
}