use bevy::prelude::*;

pub fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "The Gameeee".into(),
            resolution: (1000., 700.).into(),
            position: WindowPosition::Centered(MonitorSelection::Current), //WindowPosition::At(IVec2::new(0, 0)),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }
}