use crate::{colors::rgb_to_col, grid};
use bevy::prelude::*;
use catppuccin::PALETTE as palette;

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 480.;
pub const UPDATE_FREQ: f64 = 1.5;

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2d::default());
}

pub fn plugin(app: &mut App) {
    let window = Window {
        title: "Conway's Game of Life".into(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..default()
    };

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    });

    app.add_plugins((default_plugins, grid::plugin))
        .insert_resource(Time::<Fixed>::from_hz(UPDATE_FREQ))
        .insert_resource(ClearColor(rgb_to_col(palette.mocha.colors.crust.rgb)))
        .add_systems(Startup, setup);
}
