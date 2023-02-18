use bracket_lib::prelude::*;

use crate::{player::Player, SCREEN_HEIGHT, SCREEN_WIDTH};
 
pub struct Obstacle {
    /* world space position */
    pub x: i32,
    /* gap center */
    gap_y: i32,
    /* size of the gap */
    size: i32
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            /* current player x world-space + SCREEN_WIDTH */
            x, 
            /* random integer in range [x, y) */
            gap_y: random.range(5,25),
            /* gaps shrink as the score increase but not less than 2 */
            size: i32::max(3, 20 - score)
        }
    }

    pub fn render(& mut self, ctx: & mut BTerm, player_x: i32) {
        /* convert obstacle x world-space to x screen-space */
        let screen_x = self.x - player_x;
        let half_gap_size: i32 = self.size / 2;

        /* draw the ground */
        for x in 0..SCREEN_WIDTH {
            ctx.set(
                x,
                SCREEN_HEIGHT - 1,
                WHITE,
                WHITE,
                to_cp437('#')
            );

        }
        /* draw top half of the obstacle */
        for y in 0..(self.gap_y - half_gap_size) {
            ctx.set(
                screen_x,
                y,
                GRAY,
                GRAY,
                179
            );
        }

        /* draw bottom half of the obstacle */
        for y in (self.gap_y + half_gap_size)..(SCREEN_HEIGHT - 1) {
            ctx.set(
                screen_x,
                y,
                GRAY,
                GRAY,
                179
            );
        }
    }

    /* determine whether the player collided with the obstacle */
    pub fn collision(& self, player: & Player) -> bool {
        let half_gap_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < (self.gap_y - half_gap_size) as f32;
        let player_below_gap = player.y > (self.gap_y + half_gap_size) as f32;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
