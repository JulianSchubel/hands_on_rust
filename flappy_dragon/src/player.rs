use bracket_lib::prelude::*;

use crate::{ INIT_WORLD_SPACE, VELOCITY, TERMINAL_VELOCITY, GRAVITY_MODIFIER, WORLD_SPACE_VELOCITY, FLAP_VELOCITY, DRAGON_FRAMES };

/* player attributes */
pub struct Player {
    /* world-space */
    pub x: i32,
    pub y: f32,
    /* vertical momentum: represents gravity */
    pub velocity: f32,
    terminal_velocity: f32,
    gravity_modifier: f32,
    world_space_velocity: i32,
    flap_velocity: f32,
    animation_frame: usize,
}

impl Player {
    /* constructor */
    pub fn new(x: i32, y: f32) -> Player {
        Player {
            x,
            y,
            velocity: VELOCITY,
            terminal_velocity: TERMINAL_VELOCITY,
            gravity_modifier: GRAVITY_MODIFIER,
            world_space_velocity: WORLD_SPACE_VELOCITY,
            flap_velocity: FLAP_VELOCITY,
            animation_frame: 0,
        }
    }

    /* render the player */
    pub fn render(& mut self, ctx: & mut BTerm) {
        /* Set a single character on the screen */
        /* Can use RGB::from_u8() or RGB::from_hex() to indicate colours */
        /* 
        ctx.set(
            /* player has constant x screen-space */
            INIT_WORLD_SPACE,
            /* y screen-space */
            self.y,
            YELLOW,
            BLACK,
            /* convert unicode character to CP437 character set equivalent */
            to_cp437('@')
        ); */
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(INIT_WORLD_SPACE as f32, self.y), 
            1, 
            Degrees::new(0.0),
            PointF::new(2.0,2.0), 
            WHITE, 
            LIGHT_SKY,
            DRAGON_FRAMES[self.animation_frame],
        );
        ctx.set_active_console(0);
    }

    pub fn gravity_and_move(& mut self) {
        /* check for terminal velocity */
        if self.velocity < self.terminal_velocity {
            self.velocity += self.gravity_modifier;
        }
        self.y += self.velocity;
        self.x += self.world_space_velocity;
        self.animation_frame += 1;
        self.animation_frame = self.animation_frame % 6;
        /* cannot leave upper screen boundary */
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    pub fn flap(& mut self) {
        self.velocity = self.flap_velocity;
    }
}

