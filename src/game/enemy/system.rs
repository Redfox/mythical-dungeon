use bevy::prelude::*;
use crate::game::enemy::resources::Enemy;

pub fn enemy_resource(mut commands: Commands) {
    commands.insert_resource(Enemy::default());
}