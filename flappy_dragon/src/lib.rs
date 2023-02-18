pub const SCREEN_WIDTH: i32 = 40;
pub const SCREEN_HEIGHT: i32 = 25;

/* From duration dictates how often the game state is updated in ms */
pub const FRAME_DURATION: f32 = 60.0; 
pub const INIT_WORLD_SPACE: i32 = 5;
pub const INIT_SCREEN_SPACE: f32 = 25.0;

/* Player struct innit values */
pub const VELOCITY: f32 = 0.0;
pub const TERMINAL_VELOCITY: f32 = 2.0;
pub const GRAVITY_MODIFIER: f32 = 0.1;
pub const WORLD_SPACE_VELOCITY: i32 = 1;
pub const FLAP_VELOCITY: f32 = -0.75;

/* dragon frames for animation cycle */
pub const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];
 
pub mod game_modes;
pub mod game_state;
pub mod player;
pub mod obstacle;
