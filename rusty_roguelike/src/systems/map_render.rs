use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    /* new draw batch */
    draw_batch.target(0);
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x ..= camera.right_x {
            let pt = Point::new(x,y);
            /* Ensure that the viewport follows the player i.e. redrawing from 0,0 */
            let offset = Point::new(camera.left_x, camera.top_y);
            /* For every x,y coordinate in the camera viewport check that it is in bounds */
            if map.in_bounds(pt) {
                /* fetch the tile type from the world space */
                let index = map_index(x, y);
                /* Set the tile relative to the camera viewport */
                let glyph = match map.tiles[index] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#')
                };
                /* Add draw command to the batch */
                draw_batch.set(
                    pt - offset, 
                    ColorPair::new(
                        WHITE,
                        BLACK
                    ),
                    glyph
                );
            }
        }
    }
    /* Add the batch to the global command list with a 0 sort order */
    draw_batch.submit(0).expect("Batch error");
}
