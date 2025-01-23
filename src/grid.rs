use crate::app::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::cell::Cell;
use bevy::prelude::*;
use itertools::iproduct;

pub const CELL_SIZE: f32 = 16.;

pub const GRID_WIDTH: usize = {
    assert!(WINDOW_WIDTH % CELL_SIZE == 0.);
    (WINDOW_WIDTH / CELL_SIZE) as usize
};

pub const GRID_HEIGHT: usize = {
    assert!(WINDOW_HEIGHT % CELL_SIZE == 0.);
    (WINDOW_HEIGHT / CELL_SIZE) as usize
};

const X_SLOPE: f32 = 0.;
const Y_SLOPE: f32 = 0.;

#[derive(Resource, Clone)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Self {
        let grid = vec![vec![Cell::Dead; w]; h];

        Self { grid, w, h }
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(y).and_then(|row| row.get(x))
    }

    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[y][x] = cell;
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        self.grid[y][x] = !self.grid[y][x];
    }

    pub fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        if matches!(
            self.get(x.wrapping_sub(1), y.wrapping_sub(1)),
            Some(Cell::Alive)
        ) {
            count += 1
        }

        if matches!(self.get(x, y.wrapping_sub(1)), Some(Cell::Alive)) {
            count += 1
        }

        if matches!(
            self.get(x.wrapping_add(1), y.wrapping_sub(1)),
            Some(Cell::Alive)
        ) {
            count += 1
        }

        if matches!(self.get(x.wrapping_sub(1), y), Some(Cell::Alive)) {
            count += 1
        }

        if matches!(self.get(x.wrapping_add(1), y), Some(Cell::Alive)) {
            count += 1
        }

        if matches!(
            self.get(x.wrapping_sub(1), y.wrapping_add(1)),
            Some(Cell::Alive)
        ) {
            count += 1
        }

        if matches!(self.get(x, y.wrapping_add(1)), Some(Cell::Alive)) {
            count += 1
        }

        if matches!(
            self.get(x.wrapping_add(1), y.wrapping_add(1)),
            Some(Cell::Alive)
        ) {
            count += 1
        }

        count
    }

    pub fn clear(&mut self) {
        for (i, j) in iproduct!(0..self.w, 0..self.h) {
            self.set(i, j, Cell::Dead);
        }
    }
}

fn setup(
    mut cmd: Commands,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let (grid, (w, h)) = (grid.grid(), grid.size());

    for (i, j) in iproduct!(0..w, 0..h) {
        let mesh = meshes.add(Mesh::from(Rectangle::new(CELL_SIZE - 1., CELL_SIZE - 1.)));

        let cell = grid.get(j).and_then(|row| row.get(i)).unwrap();
        let color = match cell {
            Cell::Dead => colors.add(Color::srgb(0.1, 0.1, 0.1)),
            Cell::Alive => colors.add(Color::srgb(0.9, 0.9, 0.9)),
        };

        // slope = (output_end - output_start) / (input_end - input_start)
        // output = output_start + slope * (input - input_start)
        let transform = {
            let slope_x = WINDOW_WIDTH / (w as f32);
            let slope_y = WINDOW_HEIGHT / (h as f32);

            let x = -WINDOW_WIDTH / 2. + slope_x * i as f32;
            let y = -WINDOW_HEIGHT / 2. + slope_y * j as f32;

            Transform::from_xyz(x, y, 0.)
        };

        cmd.spawn((Mesh2d(mesh), MeshMaterial2d(color), transform));
    }
}

fn update(mut grid: ResMut<Grid>) {
    let (w, h) = grid.size();
    let old = grid.clone();
    grid.clear();

    for (i, j) in iproduct!(0..w, 0..h) {
        let neighbors = old.alive_neighbors(i, j);

        match old.get(i, j) {
            Some(Cell::Alive) if neighbors < 2 || neighbors > 3 => grid.set(i, j, Cell::Dead),
            Some(Cell::Dead) if neighbors == 3 => grid.set(i, j, Cell::Alive),
            _ => (),
        }
    }
}

pub fn plugin(app: &mut App) {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .insert_resource(grid);
}
