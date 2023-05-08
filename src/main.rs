use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::systems::spawn_camera;
use bevy_mod_picking::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::{WorldInspectorPlugin, ResourceInspectorPlugin};

mod systems;
mod main_menu;
mod game;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera);

    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new());

    app.run();
}
