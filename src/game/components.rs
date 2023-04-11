use bevy::prelude::{Color, Component};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
pub struct Player {
    pub name: String,
    pub speed: f32,
    pub color: Color,
    pub skills: Vec<String>,
}