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
        self.mode = GameMode::Menu;
        /* Clear the context */
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(7, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        /* Get keyboard input */
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => ()
            }
        }
    }

    fn dead(& mut self, ctx: & mut BTerm) -> () {
        //TODO implement
        self.mode = GameMode::End;
        /* Clear the context */
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(7, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        /* Get keyboard input */
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => ()
            }
        }
    }

    fn play(& mut self, ctx: & mut BTerm) -> () {
        //TODO implement
        self.mode = GameMode::End;
    }

    /* Ready game for playin; purging game state */
    fn restart(& mut self) -> () {
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
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
       }
   }
}


/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context, State::new())
}



