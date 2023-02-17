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
        Obstacl {
            x, 
            /* random integer in range [x, y) */
            gap_y: random.range(10,40),
            /* gaps shrink as the score increase but not less than 2 */
            size: i32::max(2, 20 - score)
        }
    }
}
