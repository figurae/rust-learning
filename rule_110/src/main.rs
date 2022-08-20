#![allow(dead_code, unused)]

use std::process::exit;

use gfx::{calculate_cell_width, modify_all_cells};
use macroquad::prelude::*;
use rule_110::determine_next_generation;

mod gfx;

// FIXME: this is too slow with higher lengths.
const GENERATION_LENGTH: usize = 240;
const NUMBER_OF_GENERATIONS: usize = 160;

#[macroquad::main("Rule 110")]
async fn main() {
    let mut generation = rule_110::initialize_first_generation(GENERATION_LENGTH);

    // TODO: restore text version, make a switch.
    // for _i in 0..NUMBER_OF_GENERATIONS {
        // rule_110::print_generation(&generation);
        // generation = rule_110::determine_next_generation(&generation);
    // }

    let mut move_y: f32 = 0.0 - calculate_cell_width();
    let mut generations: Vec<Vec<gfx::GfxCell>> = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }

        clear_background(LIGHTGRAY);

        move_y += calculate_cell_width();

        // TODO: this would greatly benefit from some caching.
        let mut generation_gfx = gfx::convert_generation_to_gfx(&generation, move_y);

        generations.push(generation_gfx);
        // dbg!(&generation_gfx);

        gfx::draw_all_generations(&generations);

        generation = determine_next_generation(&generation);
        next_frame().await;
    }
}
