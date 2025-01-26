# About

A simple [Conway's Game of Life](https://www.wikiwand.com/en/articles/Conway%27s_Game_of_Life) implementation in Rust, using Bevy.

# Installation

To run, use

```sh
cargo run --release
```

To install, use

```sh
cargo install --path .
```

# Tweaks

There are several constants that determine the behaviour of the game. Following is a list (sorted by file) explaining what tweaking each constant would do.

## `src/app.rs`

- `WINDOW_WIDTH` Changes the horizontal width of the game's window, Units: Pixels\*
- `WINDOW_HEIGHT` Changes the vertical height of the game's window, Units: Pixels\*
- `UPDATE_FREQ` Changes the number of times the board is updated, per second, Units: Hz

## `src/grid.rs`

- `CELL_SIZE` Changes the size of a cell, Units: Pixels\*
- `BORDER_SIZE` Changes the size of the border of a cell, Units: Pixels

> \*NOTE: The quantity WINDOW_WIDTH / CELL_SIZE and WINDOW_HEIGHT / CELL_SIZE must remain integral values, to avoid scaling issues. If changing the constants leads to a non-integral value for above, the program will fail to compile.
