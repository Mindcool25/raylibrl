use raylib::prelude::*;

pub mod cell;
pub mod entity;
pub mod map;
pub mod util;

use crate::entity::Entity;
use crate::map::Map;
use crate::util::mut_ref;

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

    mut_ref!(m.map[0].clone()).entity = Some(e.clone());
    mut_ref!(e).cell = Some(m.map[0].clone());

    mut_ref!(m.map[3].clone()).entity = Some(e2.clone());
    mut_ref!(e2).cell = Some(m.map[3].clone());

    use raylib::consts::KeyboardKey::*;
    while !rl.window_should_close() {
        //
        // INPUT HANDLING
        //
        if rl.is_key_pressed(KEY_K) {
            println!("UP");
            mut_ref!(e).move_up();
        }
        if rl.is_key_pressed(KEY_J) {
            println!("DOWN");
            mut_ref!(e).move_down();
        }
        if rl.is_key_pressed(KEY_L) {
            println!("RIGHT");
            mut_ref!(e).move_right();
        }
        if rl.is_key_pressed(KEY_H) {
            println!("LEFT");
            mut_ref!(e).move_left();
        }

        //
        // DRAWING
        //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        m.draw(&font, &mut d);
    }
}
