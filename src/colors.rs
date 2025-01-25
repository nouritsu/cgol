use bevy::prelude::*;

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
