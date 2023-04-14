use bevy::asset::Handle;
use bevy::prelude::{Color, Component, Image};
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, LdtkEntity)]
pub struct Player {
    pub name: String,
    pub speed: f32,
    pub color: Color,
    pub skills: Vec<String>,
    pub size: f32,
    pub texture: Handle<Image>
}