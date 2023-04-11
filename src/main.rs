#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use crate::game::GamePlugin;

mod game;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(game::config::get_window_config())
        )
        .add_plugin(GamePlugin);

    app.run();
}