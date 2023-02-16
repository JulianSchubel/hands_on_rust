use bracket_lib::prelude::*;

/* player attributes */
pub struct Player {
    /* world-space */
    pub x: i32,
    /* screen-space */
    pub y: i32,
    /* vertical momentum: represents gravity */
    velocity: f32,
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
            velocity: 0.0,
            terminal_velocity: 2.0,
            gravity_modifier: 0.2,
            world_space_velocity: 1,
            flap_velocity: -2.0,
        }
    }

    /* render the player */
    pub fn render(& mut self, ctx: & mut BTerm) {
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

