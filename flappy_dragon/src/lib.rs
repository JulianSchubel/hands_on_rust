const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

/* From duration dictates how often the game state is updated in ms */
const FRAME_DURATION: f32 = 60.0; 
const INIT_WORLD_SPACE: i32 = 5;
const INIT_SCREEN_SPACE: i32 = 25;

/* Player struct innit values */
const VELOCITY: f32 = 0.0;
const TERMINAL_VELOCITY: f32 = 2.0;
const GRAVITY_MODIFIER: f32 = 0.45;
const WORLD_SPACE_VELOCITY: i32 = 1;
const FLAP_VELOCITY: f32 = -2.0;

pub mod game_modes;
pub mod game_state;
pub mod player;
pub mod obstacle;
