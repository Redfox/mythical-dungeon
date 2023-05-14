use bevy::prelude::*;
use crate::game::GameState;

pub fn startup(mut app_state_next_state: ResMut<NextState<GameState>>,) {
    app_state_next_state.set(GameState::Map);
}