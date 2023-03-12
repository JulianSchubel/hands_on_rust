use crate::prelude::*;

#[system]
/* read access for point components */
#[read_component(Point)]
/* read access for render components */
#[read_component(Render)]
pub fn entity_render(ecs: & SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    /* Use camera position to determine entities relative position in the viewport */
    let offset = Point::new(camera.left_x, camera.top_y);
    /* Query for all entities with Point and Render components, returns entities containing both */
    <(&Point, &Render)>::query()
        /* iterate over matching entities components; query becomes an iterator over SubWorld ecs*/
        .iter(ecs)
        /* closure capturing position and render (receive query's componentsas a tuple) and adding to the draw batch */
        .for_each( |(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
        }
    );
    draw_batch.submit(5000).expect("Batch error");
}
