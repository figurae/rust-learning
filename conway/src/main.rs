#![allow(unused, dead_code)]
use std::time::Instant;

use macroquad::prelude::*;
use ndarray::Array2;

#[macroquad::main("conway")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut world = World::new(Array2::from_elem((w, h), CellState::Dead));
    let mut buffer = World::new(Array2::from_elem((w, h), CellState::Dead));

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    for cell in &mut world.board {
        if rand::gen_range(0, 5) == 0 {
            *cell = CellState::Alive;
        }
    }

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        for x in 0..w {
            for y in 0..h {
                let current_cell = world.board.get((x, y)).unwrap();
                let no_of_neighbors = calculate_neighbors(x, y, &world.board, Neighborhood::Moore);

                *buffer.board.get_mut((x, y)).unwrap() = match (
                    current_cell,
                    no_of_neighbors,
                ) {
                    (CellState::Alive, n) if n < 2 => CellState::Dead,
                    (CellState::Alive, n) if n == 2 || n == 3 => CellState::Alive,
                    (CellState::Alive, n) if n > 3 => CellState::Dead,
                    (CellState::Dead, 3) => CellState::Alive,
                    (otherwise, _) => *otherwise,
                }
            }                
        }

        let start = Instant::now();
        for x in 0..w {
            for y in 0..h {
                *world.board.get_mut((x, y)).unwrap() = *buffer.board.get((x, y)).unwrap();

                image.set_pixel(x as u32, y as u32, match buffer.board.get((x, y)).unwrap() {
                    CellState::Alive => BLACK,
                    CellState::Dead => WHITE,
                })
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await;
    }

}

// TODO: try and do this using methods too
fn calculate_neighbors(x: usize, y: usize, board: &Array2<CellState>, neighborhood: Neighborhood) -> u32 {
    let mut neighbors_count: u32 = 0;

    match neighborhood {
        Neighborhood::Moore => {
            for i in x.saturating_sub(1)..=clamp(x+1, 0, board.nrows()-1) {
                for j in y.saturating_sub(1)..=clamp(y+1, 0, board.ncols()-1) {
                    if i == x && j == y {
                        continue;
                    }

                    // TODO: this is very slow, maybe switch to 1D Vecs? or do it without Vecs?
                    match board[(i, j)] {
                        CellState::Alive => neighbors_count += 1,
                        CellState::Dead => (),
                    }
                }
            }
        },
        Neighborhood::VonNeumann => {
            todo!();
        }
    }

    neighbors_count
}

#[derive(Debug)]
struct World {
    width: usize,
    height: usize,
    board: Array2<CellState>,
}

impl World {
    fn new(board: Array2<CellState>) -> Self {
        World {
            width: board.ncols(),
            height: board.nrows(),
            board,
        }
    }
}

enum Neighborhood {
    Moore,
    VonNeumann,
}

#[derive(Clone, Copy, Debug)]
enum CellState {
    Alive,
    Dead,
}

