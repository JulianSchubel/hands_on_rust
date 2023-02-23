use crate::prelude::*;
/* apply_verical_tunnel */
use std::cmp::{ min, max };

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    /* Works on its own copy which will get passed to the game state */
    pub map: Map,
    pub rooms: Vec<Rect>,
    /* player map entry location */
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: & mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        /* Player starts in the center of a room to ensure they are on a passable map tile */
        mb.player_start = mb.rooms[0].center();
        mb
    }

    /* fill: fill the map with a specified TileType */
    fn fill(& mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /* build_random_rooms: build NUM_ROOMS non-overlapping rooms */
    fn build_random_rooms(& mut self, rng: & mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                /* rng.range is [m, n) */
                /* left justified x value */
                rng.range(1, SCREEN_WIDTH - 10),
                /* top justified y value */
                rng.range(1, SCREEN_HEIGHT - 10), 
                /* rectangle width */
                rng.range(2, 10), 
                /* rectangle height */
                rng.range(2, 10),
            );
            let mut overlap = false;
            /* check if the newly produced room overlaps with any other in the room vector */
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            /* Add the room to the room vector and set the associated map tiles as passable if it does not overlap */
            if !overlap {
                /* indicate passable map tiles for the room area */
                room.for_each(|p| {
                    /* check that the rooms x,y values are in the map boundaries */
                    /* Don't need this, it is impossible given the range min and max */
                    if ((p.x > 0) && (p.x < SCREEN_WIDTH)) && ((p.y > 0) && (p.y < SCREEN_HEIGHT)) {
                        /* find the corresponding index number in the map tiles vector */
                        let index = map_index(p.x, p.y);
                        /* update the working map.tiles vector */
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    /* apply_verical_tunnel: create a column of passable tiles */
    fn apply_vertical_tunnel(& mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1,y2)..=max(y1,y2) {
            /* indicate column x has |y2 - y1| (absolute value) passable tiles */
            if let Some(index) = self.map.try_index(Point::new(x,y)) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }

    /* apply_horizontal_tunnel: create a row of passable tiles*/
    fn apply_horizontal_tunnel(& mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            /* indicate row y has |x2 - x1| (absolute vale) passable tiles */
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }

    /* build_corridors: utilizes apply_vertical_tunnel and apply_horizontal_tunnel to produce
     * corridors between rooms */
    fn build_corridors(& mut self, rng: & mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone(); // Vec implements clone
        /* cmp() returns an indicator as to whether the two elements are ==, >, or < one another.
         * Requires a closure that provides a total ordering */
        /* order rooms by the x coordinate of their central point */
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        
        /* get points of central points of adjacent rooms and join them together with corriders */
        /* sorting the rooms shortens the length of corridors between them */
        for (i, room) in rooms.iter().enumerate().skip(1) {
            /* why we skipped the first entry */
            let prev = rooms[i-1].center();
            let new = room.center();
            /* randomly determine if we start with a horizontal or vertical corridor */
            if rng.range(0,2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
