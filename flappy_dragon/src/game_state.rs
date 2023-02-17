use bracket_lib::prelude::*;

use crate::{ SCREEN_HEIGHT, SCREEN_WIDTH, INIT_WORLD_SPACE, INIT_SCREEN_SPACE, FRAME_DURATION };
use crate::game_modes::GameMode;
use crate::player::Player;

/* game state structure */
pub struct State {
    /* player instance in the game state */
    player: Player,
    /* tracks the time accumulated between frames in ms to control the game's speed */
    frame_time: f32,
    mode: GameMode,
}

/* State associated functions */
impl State {
    /* constructor */
    pub fn new() -> State {
        State {
            player: Player::new(INIT_WORLD_SPACE, INIT_SCREEN_SPACE),
            frame_time: 0.0,
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
        /* set context background colour */
        ctx.cls_bg(NAVY);
        /* tick() runs as fast as possible, slow game speed down by only updating after
         * FRAME_DURATION has elapsed */
        /* accumulate the time since the last tick function: ctx.frame_time_ms is the tme since the last tick() function in ms */
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time >= FRAME_DURATION {
            self.frame_time = 0.0;
            /* update the game state */
            self.player.gravity_and_move();
        }
        /* flap on spacebar */
        /* do not restrict by frame time as keyboard will be unresponsive during "wait" frames */
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        /* render the player to the screen */
        self.player.render(ctx);
        ctx.print(0,0, "Press SPACE to flap.");
        /* check if player has fallen off bottom of screen, i.e. hit the ground */
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    /* Ready game for playing; purging game state */
    pub fn restart(&mut self)  {
        /* reset the player */
        self.player = Player::new(INIT_WORLD_SPACE, INIT_SCREEN_SPACE);
        /* reset the frame time */
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}

/* GameState requires that the object implement a tick() function */
impl GameState for State {
    /* represents a single frame or execution of the game loop */
    fn tick(&mut self, ctx: &mut BTerm) {
        /* check game mode to determine tick() behaviour */
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
