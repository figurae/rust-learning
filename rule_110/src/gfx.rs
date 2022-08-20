#![allow(dead_code, unused)]

use macroquad::prelude::*;
use rule_110::{Cell, State};

// TODO: why not &[&[GfxCell]]?
pub fn draw_all_generations(generations: &Vec<Vec<GfxCell>>) {
    for generation in generations {
        draw_generation(generation)
    }
}

pub fn draw_generation(generation: &[GfxCell]) {
    for cell in generation {
        draw_cell(cell)
    }
}

pub fn convert_generation_to_gfx(generation: &[Cell], move_y: f32) -> Vec<GfxCell> {
    let mut generation_gfx: Vec<GfxCell> = Vec::new();

    let mut pos_x: f32 = 0.0;
    let width = calculate_cell_width();

    for cell in generation {
        generation_gfx.push(GfxCell {
            rect: Rect {
                pos: Vec2::new(pos_x, move_y),
                ..Default::default()
            },
            state: cell.state,
        });

        pos_x += width;
    }

    generation_gfx
}

pub fn modify_all_cells(amount: f32, generation: &mut [GfxCell]) {
    for cell in generation {
        cell.modify_y(amount);
    }
}

fn draw_cell(cell: &GfxCell) {
    draw_rectangle(
        cell.rect.pos.x,
        cell.rect.pos.y,
        cell.rect.side,
        cell.rect.side,
        match cell.state {
            // TODO: abstract this away.
            State::Dead => LIGHTGRAY,
            State::Alive => RED,
        },
    )
}

#[derive(Debug)]
pub struct GfxCell {
    pub rect: Rect,
    pub state: State,
}

#[derive(Debug)]
pub struct Rect {
    pub pos: Vec2,
    pub side: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            pos: Vec2::new(0.0, 0.0),
            side: calculate_cell_width(),
        }
    }
}

impl Default for GfxCell {
    fn default() -> Self {
        GfxCell {
            rect: Rect::default(),
            state: State::Dead,
        }
    }
}

impl GfxCell {
    // TODO: I don't think this should be necessary at all.
    pub fn modify_y(&mut self, amount: f32) {
        self.rect.pos.y += amount;
    }
}

// TODO: cache the result, change only if screen_width() changes.
pub fn calculate_cell_width() -> f32 {
    screen_width() / crate::GENERATION_LENGTH as f32
}
