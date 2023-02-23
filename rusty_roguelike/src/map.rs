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
}


/* map_idx: utilizes striding to return the index of a one dimensional vector corresponding to a (x,y) co-ordinate (i.e. as if it were a row-first encoded matrix) */
pub fn map_index(x: i32, y: i32) -> usize {
   ((y * SCREEN_WIDTH) + x) as usize
}

/* map_crd: returns the reciprocal of a striding transformation, returning an (x, y) co-ordinate tuple corresponding to a row-first encoded matrix from a one dimensional vector index */
pub fn map_coordinates(index: usize) -> (i32, i32) {
    ((index % SCREEN_WIDTH as usize) as i32, (index / SCREEN_WIDTH as usize) as i32)
}

