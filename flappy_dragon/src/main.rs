#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

/* game modes */
enum GameMode {
    Menu,
    Playing,
    End
}

/* game state structure */
struct State {
    mode: GameMode,
}

/* State associated functions */
impl State {
    fn new() -> State {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(& mut self, ctx: & mut BTerm) -> () {
        /* Clear the context */
        self.mode = GameMode::Menu;
    }

    fn dead(& mut self, ctx: & mut BTerm) -> () {
        //TODO implement
        self.mode = GameMode::End;
    }

    fn play(& mut self, ctx: & mut BTerm) -> () {
        //TODO implement
        self.mode = GameMode::Playing;
    }

    /* Ready game for playin; purging game state */
    fn restart(& mut self, ctx: & mut BTerm) -> () {
        self.mode = GameMode::Playing;
    }
}


/* implement the GameState trait for State */
/* GameState requires that the object implement a tick() function */
impl GameState for State {
   fn tick (&mut self, ctx: &mut BTerm) {
       /* check game mode */
       match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.dead(ctx),
            GameMode::End => self.play(ctx),
       }
   }
}


/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}



