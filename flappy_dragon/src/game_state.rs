use bracket_lib::prelude::*;
use crate::game_modes::GameMode;

/* game state structure */
pub struct State {
    mode: GameMode,
}

/* State associated functions */
impl State {
    /* constructor */
    pub fn new() -> State {
        State {
            mode: GameMode::Menu,
        }
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm)  {
        self.mode = GameMode::Menu;
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(7, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        /* Get keyboard input */
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => (),
            }
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm)  {
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
                _ => (),
            }
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm)  {
        //TODO implement
        self.mode = GameMode::End;
    }

    /* Ready game for playin; purging game state */
    pub fn restart(&mut self)  {
        self.mode = GameMode::Playing;
    }
}

/* GameState requires that the object implement a tick() function */
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        /* check game mode to determine tick() behaviour */
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
