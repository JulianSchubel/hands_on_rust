#![warn(clippy::all, clippy::pedantic)]


use rusty_roguelike::prelude::*;

fn main() -> BError {
    /* with_fps_cap() automatically tracks the game speed and tells the OS it can rest the CPU between frames */
    let context = BTermBuilder::simple80x50() 
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
