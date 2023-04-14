#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::game::GamePlugin;

mod game;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(game::config::get_window_config())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
        )
        .add_plugin(LdtkPlugin)
        .add_plugin(GamePlugin);

    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}
