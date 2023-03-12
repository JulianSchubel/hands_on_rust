use crate::prelude::*;

mod player_input;
mod map_render;
mod entity_render;

pub fn schedule_builder() -> Schedule {
    /* Create a Lgeion scheduler: an execution plan for systems */
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()

}


