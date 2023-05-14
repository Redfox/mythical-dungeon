use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

#[derive(Resource, Clone, Debug, FromReflect, Reflect, InspectorOptions)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub health: f32,
    pub skills: Vec<String>
}

#[derive(Resource, Clone, Debug, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Player {
    pub characters: Vec<Character>
}

impl Default for Player {
    fn default() -> Self {
        Self {
            characters: vec![
                Character {
                    name: "Medusa 1".to_string(),
                    level: 1,
                    health: 100.,
                    skills: vec!["FireBall".to_string(), "Arrow".to_string()]
                },
                Character {
                    name: "Medusa 2".to_string(),
                    level: 1,
                    health: 70.,
                    skills: vec!["Arrow".to_string()]
                }
            ]
        }
    }
}
