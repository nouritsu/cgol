use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    cell::Cell,
    colors::{rgb_to_col, Colors},
    index::Index,
};
use bevy::{prelude::*, window::PrimaryWindow};
use catppuccin::PALETTE as palette;
use itertools::iproduct;
use map_range::MapRange;

pub const CELL_SIZE: f32 = 16.;
pub const BORDER_SIZE: f32 = 1.;

pub const GRID_WIDTH: usize = {
    assert!(
        WINDOW_WIDTH % CELL_SIZE == 0.,
        "Window width must be divisible by cell size"
    );
    (WINDOW_WIDTH / CELL_SIZE) as usize
};

pub const GRID_HEIGHT: usize = {
    assert!(
        WINDOW_HEIGHT % CELL_SIZE == 0.,
        "Window height must be divisible by cell size"
    );
    (WINDOW_HEIGHT / CELL_SIZE) as usize
};

#[derive(Resource, Clone, Debug)]
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let (w, h) = grid.size();

    let mut res_colors = Colors::new();
    res_colors.push(colors.add(rgb_to_col(palette.mocha.colors.base.rgb)));
    res_colors.push(colors.add(rgb_to_col(palette.mocha.colors.mauve.rgb)));

    for (i, j) in iproduct!(0..w, 0..h) {
        let mesh = {
            let rec = Rectangle::new(CELL_SIZE - BORDER_SIZE, CELL_SIZE - BORDER_SIZE);
            Mesh2d(meshes.add(Mesh::from(rec)))
        };

        let material = MeshMaterial2d(res_colors.get(0).expect("infallible"));

        let transform = {
            let x_input_start = 0.;
            let y_input_start = 0.;

            let x_input_end = (w - 1) as f32;
            let y_input_end = (h - 1) as f32;

            let x_output_start = -WINDOW_WIDTH / 2. + CELL_SIZE;
            let y_output_start = -WINDOW_HEIGHT / 2. + CELL_SIZE;

            let x_output_end = WINDOW_WIDTH / 2. - CELL_SIZE;
            let y_output_end = WINDOW_HEIGHT / 2. - CELL_SIZE;

            let x = (i as f32).map_range(x_input_start..x_input_end, x_output_start..x_output_end);
            let y = (j as f32).map_range(y_input_start..y_input_end, y_output_start..y_output_end);

            Transform::from_xyz(x, y, 0.)
        };

        let index = Index::new(i, j);

        cmd.spawn((mesh, material, transform, index));
    }

    cmd.insert_resource(grid);
    cmd.insert_resource(res_colors);
}

fn update(mut grid: ResMut<Grid>) {
    let (w, h) = grid.size();
    let old = grid.clone();

    for (i, j) in iproduct!(0..w, 0..h) {
        let neighbors = old.alive_neighbors(i, j);

        match old.get(i, j) {
            Some(Cell::Alive) if neighbors < 2 || neighbors > 3 => grid.set(i, j, Cell::Dead),
            Some(Cell::Dead) if neighbors == 3 => grid.set(i, j, Cell::Alive),
            _ => (),
        }
    }
}

fn render(
    grid: Res<Grid>,
    colors: Res<Colors>,
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &Index)>,
) {
    for (mut material, &Index(i, j)) in query.iter_mut() {
        let cell = grid.get(i, j).expect("infallible");

        let color = colors
            .get(match *cell {
                Cell::Dead => 0,
                Cell::Alive => 1,
            })
            .expect("infallible");

        *material = MeshMaterial2d(color);
    }
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut grid: ResMut<Grid>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cells: Query<(&Transform, &Index)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Some(position) = windows.single().cursor_position() else {
        return;
    };

    let position = {
        let x_input_start = 0.;
        let y_input_start = 0.;

        let x_input_end = WINDOW_WIDTH;
        let y_input_end = WINDOW_HEIGHT;

        let x_output_start = -WINDOW_WIDTH / 2.;
        let y_output_start = WINDOW_HEIGHT / 2.;

        let x_output_end = WINDOW_WIDTH / 2.;
        let y_output_end = -WINDOW_HEIGHT / 2.;

        let x = position
            .x
            .map_range(x_input_start..x_input_end, x_output_start..x_output_end);

        let y = position
            .y
            .map_range(y_input_start..y_input_end, y_output_start..y_output_end);

        Vec2::new(x, y)
    };

    for (transform, index) in cells.iter() {
        let transform = transform.translation.xy();
        let Index(i, j) = *index;

        let (x_lb, x_ub) = (transform.x - CELL_SIZE / 2., transform.x + CELL_SIZE / 2.);
        let (y_lb, y_ub) = (transform.y - CELL_SIZE / 2., transform.y + CELL_SIZE / 2.);

        if (x_lb..x_ub).contains(&position.x) && (y_lb..y_ub).contains(&position.y) {
            grid.set(i, j, Cell::Alive);
        }
    }
}

pub fn plugin(app: &mut App) {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    app.add_systems(Startup, setup)
        .add_systems(Update, (handle_click, render))
        .add_systems(FixedUpdate, update)
        .insert_resource(grid);
}
