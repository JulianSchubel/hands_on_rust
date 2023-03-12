use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/*  Copy: Changes default of assignment from "move" to "copy", i.e. deep copy
    Clone: provides .clone() method which yields a deep copy of the object 
    PartialEq: Allows comparison by the == operator
 */
#[derive( Copy, Clone, PartialEq )]
pub enum TileType {
    Wall, 
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }
    
    /* in_bounds: checks whether a point is within screen boundaries */
    /* Point structure is stack allocated so no real cost to copy */
    pub fn in_bounds(& self, point: Point) -> bool {
        (point.x >= 0 && point.x < SCREEN_WIDTH) && (point.y >= 0 && point.y < SCREEN_HEIGHT) 
    }

    /* can_enter_tile: checks whether the tile type is valid for player movement */
    /* Point structure is stack allocated so no real cost to copy */
    pub fn can_enter_tile(& self, point: Point) -> bool {
        self.in_bounds(point) && (self.tiles[map_index(point.x, point.y)]==TileType::Floor)
    }

    /* try_idx: checks whether a given point is in bounds if so returns Some(map_idx(x,y)), else returns None */
    pub fn try_index(& self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None
        } else {
            return Some(map_index(point.x, point.y))
        }
    }


    /* OLD RENDER FUNCTIONS */
    /*
    pub fn render(& self, ctx: & mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_index(x, y);
                match self.tiles[index] {
                    TileType::Floor => ctx.set(x, y, GRAY, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, PURPLE, BLACK, to_cp437('#'))
                }
            }
        }
    }
    */

    /* render_viewport: uses camera boundaries to render only the visible part of the map */
    /*
    pub fn render_viewport(& self, ctx: & mut BTerm, camera: & Camera) {
        /* Render to the base map */
        ctx.set_active_console(0);
        for y in camera.top_y .. camera.bottom_y {
            for x in camera.left_x .. camera.right_x {
                /* For every x,y coordinate in the camera viewport check that it is in bounds */
                if self.in_bounds(Point::new(x,y)) {
                    /* fetch the tile type from the world space */
                    let index = map_index(x, y);
                    /* Set the tile relative to the camera viewport */
                    match self.tiles[index] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }
    */
}


/* map_idx: utilizes striding to return the index of a one dimensional vector corresponding to a (x,y) co-ordinate (i.e. as if it were a row-first encoded matrix) */
pub fn map_index(x: i32, y: i32) -> usize {
   ((y * SCREEN_WIDTH) + x) as usize
}

/* map_crd: returns the reciprocal of a striding transformation, returning an (x, y) co-ordinate tuple corresponding to a row-first encoded matrix from a one dimensional vector index */
pub fn map_coordinates(index: usize) -> (i32, i32) {
    ((index % SCREEN_WIDTH as usize) as i32, (index / SCREEN_WIDTH as usize) as i32)
}

