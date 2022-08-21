#![allow(unused, dead_code)]
use macroquad::prelude::*;
use ndarray::Array2;

#[macroquad::main("conway")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cell_array = Array2::from_elem((w, h), CellState::Dead);

    // let mut cells = Board::new(cell_array);
    let mut buffer = Board::new(Array2::from_elem((w, h), CellState::Dead));

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    for mut window in cell_array.windows((3, 3)) {
        if rand::gen_range(0, 5) == 0 {
            window[(1,1)] = CellState::Alive;
        }
    }

    dbg!(&cell_array);

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        next_frame().await;
    }

}

fn calculate_neighbors(cells: &[CellState], neighborhood: Neighborhood) -> u32 {
    let mut neighbors_count: u32 = 0;

    match neighborhood {
        Neighborhood::Moore => {
            todo!();
        },
        Neighborhood::VonNeumann => {
            todo!();
        }
    }

    neighbors_count
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    board: Array2<CellState>,
}

impl Board {
    fn new(mut board: Array2<CellState>) -> Self {
        Board {
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellState {
    Alive,
    Dead,
}

