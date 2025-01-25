use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy)]
pub struct Index(pub usize, pub usize);

impl Index {
    pub fn new(i: usize, j: usize) -> Self {
        Self(i, j)
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<Index>();
}
