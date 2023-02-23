#![warn(clippy::all, clippy::pedantic)]


use rusty_roguelike::prelude::*;

fn main() -> BError {
    /* with_fps_cap() automatically tracks the game speed and tells the OS it can rest the CPU between frames */
    let context = BTermBuilder::new() //create a generic terminal
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //size of subsequently added consoles
        .with_tile_dimensions(32, 32) //size of each character in the font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) //name of the font file to load and the character dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") //add a console with a graphics file
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // add another console with no background so it is transparent and background console show through it.
        .build()?;
    main_loop(context, State::new())
}
