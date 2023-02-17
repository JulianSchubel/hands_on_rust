struct Obstacle {
    /* world space position */
    x: i32,
    /* gap center */
    gap_y: i32,
    /* size of the gap */
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            /* current player x world-space + SCREEN_WIDTH */
            x, 
            /* random integer in range [x, y) */
            gap_y: random.range(10,40),
            /* gaps shrink as the score increase but not less than 2 */
            size: i32::max(2, 20 - score)
        }
    }

    fn render(& mut self, ctx: & mut BTerm, player_x: i32) {
        /* convert obstacle x world-space to x screen-space */
        let screen_x = self.x - player_x;
        let half_gap_size = self.size / 2;

        /* draw top half of the obstacle */
        for y in 0..self.gap_y - half_gap_size {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
            );
        }

        /* draw bottom half of the obstacle */
        for y in 0..self.gap_y + half_gap_size {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
            );
        }
    }
}
