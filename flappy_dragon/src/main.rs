#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

//use std::time::Duration;
use rodio::{ OutputStream, Sink };
//use rodio::source::{ SineWave, Source };

use flappy_dragon::*;
/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    /*
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    */
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console( SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console( SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon")
        .with_tile_dimensions(16, 16)
        .build()?;

    /* Main game audio */
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    main_loop(context, game_state::State::new(sink))
} 
