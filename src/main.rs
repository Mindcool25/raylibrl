use raylib::prelude::*;

pub mod util;

static CELL_AMOUNT: i32 = 200;

static CHAR_SIZE: f32 = 15.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("raylib RL").build();

    let _font = rl
        .load_font(&thread, "./assets/Px437_IBM_EGA_8x8.ttf")
        .expect("Failed to laod font");
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
