mod battle;

use bevy::prelude::*;
use crate::game::ui::battle::BattleUIPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BattleUIPlugin);
    }
}