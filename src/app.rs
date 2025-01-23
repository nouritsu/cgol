use bevy::prelude::*;

use crate::grid;

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 480.;

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2d::default());
}

pub fn plugin(app: &mut App) {
    let window = Window {
        title: "cgol".into(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..default()
    };

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    });

    app.add_plugins((default_plugins, grid::plugin))
        .add_systems(Startup, setup);
}
