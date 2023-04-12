use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_ecs_ldtk::{LdtkSettings, SetClearColor};
use crate::game::battle::BattlePlugin;
use crate::game::player::PlayerPlugin;
use bevy_ecs_ldtk::prelude::*;

pub(crate) mod config;
mod player;
mod components;
mod battle;

pub struct GamePlugin;

#[derive(Resource)]
struct WinSize {
    height: f32,
    width: f32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(LevelSelection::Index(0))
            .add_plugin(PlayerPlugin)
            .add_plugin(BattlePlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());

    let mut window = windows.single_mut();
    let (win_w, win_h) = (window.width(), window.height());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk.ldtk"),
        transform: Transform::from_xyz(-win_w / 2., -win_h / 2., 0.0),
        ..Default::default()
    });

    let win_size = WinSize {
        height: win_h,
        width: win_w,
    };
    commands.insert_resource(win_size);
}