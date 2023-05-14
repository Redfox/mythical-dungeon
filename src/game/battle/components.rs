use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use crate::game::player::resources::Character;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Debug)]
pub struct CharacterComponent {
    pub data: Character
}

#[derive(Component, Debug, Reflect, InspectorOptions)]
pub struct EnemyComponent {
    pub max_hp: f32,
    pub hp: f32,
}

#[derive(Component, Debug)]
pub struct HealthBar { }

#[derive(Component, Debug)]
pub struct HealthBarFill { }

#[derive(Component, Debug)]
pub struct CharSkill { }

#[derive(Component, Debug)]
pub struct Skill { }
