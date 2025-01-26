use bevy::prelude::*;
use catppuccin::Rgb;

#[derive(Resource, Clone)]
pub struct Colors(Vec<Handle<ColorMaterial>>);

impl Colors {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, col: Handle<ColorMaterial>) {
        self.0.push(col)
    }

    pub fn get(&self, i: usize) -> Option<Handle<ColorMaterial>> {
        self.0.get(i).map(|h| h.clone_weak())
    }
}

// Map RGB
pub fn rgb_to_col(Rgb { r, g, b }: Rgb) -> Color {
    Color::srgb(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
}
