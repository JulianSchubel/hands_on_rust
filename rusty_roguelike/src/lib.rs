pub mod map;
pub mod player;
pub mod game_state;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::game_state::*;
    pub use crate::player::*;
}
