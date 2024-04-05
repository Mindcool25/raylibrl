use raylib::prelude::*;

pub mod cell;
pub mod entity;
pub mod map;
pub mod util;

use crate::cell::Cell;
use crate::entity::Entity;
use crate::map::Map;
use crate::util::{borrow_cell, mut_cell, mut_ref};

static CELL_AMOUNT: i32 = 200;

static CHAR_SIZE: f32 = 15.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("raylib RL").build();

    let font = rl
        .load_font(&thread, "./assets/Px437_IBM_EGA_8x8.ttf")
        .expect("Failed to laod font");

    let e = Entity::new();

    let m: Map = Map::new();
    println!("Map: {:?}", m);
    println!("Cell: {:?}", m.map);
    m.init_cells();

    mut_ref!(m.map[0].clone()).entity = Some(e.clone());
    mut_ref!(e).cell = Some(m.map[0].clone());

    use raylib::consts::KeyboardKey::*;
    while !rl.window_should_close() {
        //
        // INPUT HANDLING
        //
        if rl.is_key_pressed(KEY_UP) {
            println!("UP");
            if borrow_cell!(e).up.is_some() {
                println!("Can move up!");
                mut_cell!(e).entity = None;
                let temp_cell = borrow_cell!(e).up.clone();
                mut_ref!(e).cell = temp_cell;
                mut_cell!(e).entity = Some(e.clone());
            }
        }
        if rl.is_key_pressed(KEY_DOWN) {
            println!("DOWN");
            if borrow_cell!(e).down.is_some() {
                println!("Can move down!");
                mut_cell!(e).entity = None;
                let temp_cell = borrow_cell!(e).down.clone();
                mut_ref!(e).cell = temp_cell;
                mut_cell!(e).entity = Some(e.clone());
            }
        }
        if rl.is_key_pressed(KEY_RIGHT) {
            println!("RIGHT");
            if borrow_cell!(e).right.is_some() {
                println!("Can move right!");
                mut_cell!(e).entity = None;
                let temp_cell = borrow_cell!(e).right.clone();
                mut_ref!(e).cell = temp_cell;
                mut_cell!(e).entity = Some(e.clone());
            }
        }
        if rl.is_key_pressed(KEY_LEFT) {
            println!("LEFT");
            if borrow_cell!(e).left.is_some() {
                println!("Can move left!");
                mut_cell!(e).entity = None;
                let temp_cell = borrow_cell!(e).left.clone();
                mut_ref!(e).cell = temp_cell;
                mut_cell!(e).entity = Some(e.clone());
            }
        }

        //
        // DRAWING
        //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        m.draw(&font, &mut d);
    }
}
