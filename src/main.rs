use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

const CELL_SIZE: f32 = 16.;

const GRID_WIDTH: usize = (WINDOW_WIDTH / CELL_SIZE) as usize;
const GRID_HEIGHT: usize = (WINDOW_HEIGHT / CELL_SIZE) as usize;

fn main() {
    let window = Window {
        title: "cgol".into(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..default()
    };

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    });

    App::new().add_plugins(default_plugins).run();
}
