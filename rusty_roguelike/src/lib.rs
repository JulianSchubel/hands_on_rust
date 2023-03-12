pub mod map;
pub mod map_builder;
pub mod game_state;
pub mod camera;
pub mod components;
/* hanldes spawning entities */
pub mod spawner;
pub mod systems;

pub mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    /* The display width / height is the viewport size */
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::game_state::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}
