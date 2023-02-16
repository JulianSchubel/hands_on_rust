#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

/* player attributes */
struct Player {
    /* world-space */
    x: i32,
    /* screen-space */
    y: i32,
    /* vertical momentum: represents gravity */
    velocity: f32,
    terminal_velocity: f32,
    gravity_modifier: f32,
    world_space_velocity: i32,
}

impl Player {
    /* constructor */
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
            terminal_velocity: 2.0,
            gravity_modifier: 0.2,
            world_space_velocity: 1,
        }
    }

    /* render the player */
    fn render(& mut self, ctx: & mut BTerm) {
        /* Set a single character on the screen */
        /* Can use RGB::from_u8() or RGB::from_hex() to indicate colours */
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            /* convert unicode character to CP437 character set equivalent */
            to_cp437('@')
        );
    }

    fn gravity_and_move(& mut self) {
        /* check for terminal velocity */
        if self.velocity < self.terminal_velocity {
            self.velocity += self.gravity_modifier;
        }
        self.y += self.velocity as i32;
        self.x += self.world_space_velocity;
        /* cannot leave upper screen boundary */
        if self.y < 0 {
            self.y = 0;
        }
    }
}

/* available game modes */
enum GameMode {
    Menu,
    Playing,
    End,
}

/* game state structure */
struct State {
    mode: GameMode,
}

/* State associated functions */
impl State {
    /* constructor */
    fn new() -> State {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) -> () {
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

    fn dead(&mut self, ctx: &mut BTerm) -> () {
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

    fn play(&mut self, ctx: &mut BTerm) -> () {
        //TODO implement
        self.mode = GameMode::End;
    }

    /* Ready game for playin; purging game state */
    fn restart(&mut self) -> () {
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

/* BError is a Result type exposed by bracket lib */
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
