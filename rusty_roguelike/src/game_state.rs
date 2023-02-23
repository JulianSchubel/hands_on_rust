use crate::prelude::*;

pub struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
   pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(& mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
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
        self.player.update(ctx, &self.map, & mut self.camera);
        self.map.render_viewport(ctx, & self.camera);
        self.player.render(ctx, & self.camera);
    }
}
