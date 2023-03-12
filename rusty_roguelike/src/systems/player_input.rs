use crate::prelude::*;

#[system]
/* define the system's component access */
#[write_component(Point)]
#[read_component(Player)]
/* #[resource] requests access to type stored in Legion's resource handler */
/* A subworld is subset of components determined by a query */
pub fn player_input(ecs: &mut SubWorld, #[resource] map: & Map, #[resource] key: &Option<VirtualKeyCode>, #[resource] camera: & mut Camera) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1,0),
            VirtualKeyCode::A => Point::new(-1,0),
            VirtualKeyCode::Right => Point::new(1,0),
            VirtualKeyCode::D => Point::new(1,0),
            VirtualKeyCode::Up => Point::new(0,-1),
            VirtualKeyCode::W => Point::new(0,-1),
            VirtualKeyCode::Down => Point::new(0,1),
            VirtualKeyCode::S => Point::new(0,1),
            _ => Point::new(0,0)
        };

        println!("{:#?}", delta);
        /* one of left, right, up, down, pressed */
        if delta.x != 0 || delta.y != 0 {
            /* query for entities with the selected components */
            let mut players = <& mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    /* Update the camera viewport */
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
