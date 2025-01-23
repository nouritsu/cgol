use bevy::prelude::*;
use cgol::app;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    App::new().add_plugins(app::plugin).run();
}
