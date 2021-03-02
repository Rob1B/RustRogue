use std::cmp::{max, min};

use rltk::RandomNumberGenerator;

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h,
        }
    }

    /// Return true if one rect intersects the other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// Return the center of the rect
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

pub fn xy_idx(x: i32, y: i32, _h: u32, w: u32) -> usize {
    assert!(x >= 0 && y >= 0);
    (y * w as i32 + x) as usize
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType], height: u32, width: u32) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y, height, width)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32, h: u32, w: u32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y, h, w);
        if idx > 0 && idx < (h * w) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32, h: u32, w: u32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y, h, w);
        if idx > 0 && idx < (h * w) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Map with solid boundaries
pub fn new_map_rooms_and_corridors(width: u32, height: u32) -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; (height * width) as usize];
    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, width as i32 - w - 1) - 1;
        let y = rng.roll_dice(1, height as i32 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false;
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map, height, width);
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y, height, width);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x, height, width);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x, height, width);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y, height, width);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}
