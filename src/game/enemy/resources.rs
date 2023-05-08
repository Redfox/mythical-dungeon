use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Enemy{
    pub name: String,
    pub level: u32,
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            name: "Beholder".to_string(),
            level: 1,
            health: 100.
        }
    }
}