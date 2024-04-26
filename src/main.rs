use raylib::prelude::*;

use crate::util::BorrowObj;

pub mod cell;
pub mod entity;
pub mod map;
pub mod util;

static CELL_AMOUNT: i32 = 200;

static CHAR_SIZE: f32 = 15.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("raylib RL").build();

    let font = rl
        .load_font(&thread, "./assets/BungeeSpice-Regular.ttf")
        .expect("Failed to load font");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
