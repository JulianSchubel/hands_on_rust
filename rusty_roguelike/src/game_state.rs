use crate::prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(& mut rng);
        /* Adds player entity and its componenets to the ECS */
        spawn_player(& mut ecs, map_builder.player_start);
        /* Spawn a monster in the center of every room except the first */
        map_builder.rooms.iter().skip(1).map(|r| r.center()).for_each(|pos| spawn_monster(& mut ecs, & mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: schedule_builder()
        }
    }
}

impl GameState for State {
   fn tick(& mut self, ctx: & mut BTerm) {
        /* clear the base map */
        ctx.set_active_console(0);
        ctx.cls();
        /* clear the player map */
        ctx.set_active_console(1);
        ctx.cls();
        /* Add keyboard stat as a resource */
        self.resources.insert(ctx.key);
        self.systems.execute(& mut self.ecs, & mut self.resources);
        /* Render all buffered draw batches */
        render_draw_buffer(ctx).expect("Render error")
    }
}
