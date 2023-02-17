use bracket_lib::prelude::*;

use crate::{ INIT_WORLD_SPACE, VELOCITY, TERMINAL_VELOCITY, GRAVITY_MODIFIER, WORLD_SPACE_VELOCITY, FLAP_VELOCITY };

/* player attributes */
pub struct Player {
    /* world-space */
    pub x: i32,
    pub y: i32,
    /* vertical momentum: represents gravity */
    pub velocity: f32,
    terminal_velocity: f32,
    gravity_modifier: f32,
    world_space_velocity: i32,
    flap_velocity: f32,
}

impl Player {
    /* constructor */
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: VELOCITY,
            terminal_velocity: TERMINAL_VELOCITY,
            gravity_modifier: GRAVITY_MODIFIER,
            world_space_velocity: WORLD_SPACE_VELOCITY,
            flap_velocity: FLAP_VELOCITY,
        }
    }

    /* render the player */
    pub fn render(& mut self, ctx: & mut BTerm) {
        /* Set a single character on the screen */
        /* Can use RGB::from_u8() or RGB::from_hex() to indicate colours */
        ctx.set(
            /* player has constant x screen-space */
            INIT_WORLD_SPACE,
            /* y screen-space */
            self.y,
            YELLOW,
            BLACK,
            /* convert unicode character to CP437 character set equivalent */
            to_cp437('@')
        );
    }

    pub fn gravity_and_move(& mut self) {
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

    pub fn flap(& mut self) {
        self.velocity = self.flap_velocity;
    }
}

