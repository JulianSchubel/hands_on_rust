#![warn(clippy::all, clippy::pedantic)]


use bracket_lib::prelude::*;
use flappy_dragon::*;

/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, game_state::State::new())
}
