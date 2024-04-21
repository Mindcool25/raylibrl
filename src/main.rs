use raylib::prelude::*;

pub mod cell;
pub mod entity;
pub mod map;
pub mod schedule;
pub mod util;

use crate::entity::Entity;
use crate::map::Map;
use crate::schedule::{Event, Schedule};
use crate::util::{borrow_cell, mut_ref};

static CELL_AMOUNT: i32 = 200;

static CHAR_SIZE: f32 = 15.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("raylib RL").build();

    let font = rl
        .load_font(&thread, "./assets/Px437_IBM_EGA_8x8.ttf")
        .expect("Failed to laod font");

    let e = Entity::new();

    let e2 = Entity::create('e', Color::BEIGE);
    let m: Map = Map::new();
    m.init_cells();

    m.map[0].clone().borrow_mut().entity = Some(e.clone());
    e.clone().as_ref().borrow_mut().cell = Some(m.map[0].clone());

    m.map[3].borrow_mut().entity = Some(e2.clone());
    e2.clone().as_ref().borrow_mut().cell = Some(m.map[3].clone());

    let mut s = Schedule::new();

    s.events.push(Event {
        owner: Some(e.clone()),
        tick: 23,
    });
    s.events.push(Event {
        owner: Some(e2.clone()),
        tick: 33,
    });
    s.events.push(Event {
        owner: Some(e.clone()),
        tick: 3,
    });
    s.events.push(Event {
        owner: Some(e.clone()),
        tick: 2,
    });

    println!("Schedule {:?}", &s);
    s.next_tick();
    println!("Schedule {:?}", &s);

    use raylib::consts::KeyboardKey::*;
    while !rl.window_should_close() {
        //
        // INPUT HANDLING
        //
        if rl.is_key_pressed(KEY_K) {
            mut_ref!(e).move_north();
        }
        if rl.is_key_pressed(KEY_J) {
            mut_ref!(e).move_south();
        }
        if rl.is_key_pressed(KEY_L) {
            mut_ref!(e).move_east();
        }
        if rl.is_key_pressed(KEY_H) {
            mut_ref!(e).move_west();
        }
        if rl.is_key_pressed(KEY_U) {
            mut_ref!(e).move_ne();
        }
        if rl.is_key_pressed(KEY_Y) {
            mut_ref!(e).move_nw();
        }
        if rl.is_key_pressed(KEY_B) {
            mut_ref!(e).move_sw();
        }
        if rl.is_key_pressed(KEY_N) {
            mut_ref!(e).move_se();
        }
        if rl.is_key_pressed(KEY_G) {
            s.next_tick();
        }

        //
        // DRAWING
        //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        m.draw(&font, &mut d);
    }
}
