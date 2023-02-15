#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

/* game state structure */
struct State {}

/* implement the GameState trait for State */
/* GameState requires that the object implement a tick() function */
impl GameState for State {
   fn tick (&mut self, ctx: &mut BTerm) {
       ctx.cls();
       ctx.print(1,1,"Hello, Bracket Terminal");
   }
}

/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State{})
}



