use bevy::prelude::*;
use crate::AppState;
use crate::main_menu::buttons_interactions::interact_with_play_button;
use crate::main_menu::components::MainMenu;
use crate::main_menu::systems::{despawn_main_menu, spawn_main_menu};

mod systems;
mod styles;
mod components;
mod buttons_interactions;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(interact_with_play_button.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}