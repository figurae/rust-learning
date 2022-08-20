#![allow(unused, dead_code)]
use macroquad::prelude::*;

#[macroquad::main("conway")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cells = vec![CellState::Dead; w * h];
    let mut buffer = vec![CellState::Dead; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    for y in 0..h {
        for x in 0..w {
            if rand::gen_range(0, 5) == 0 {
                cells[y * w + x] = CellState::Alive;
            }
        }
    }

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

enum Neighborhood {
    Moore,
    VonNeumann,
}

#[derive(Clone)]
enum CellState {
    Alive,
    Dead,
}

