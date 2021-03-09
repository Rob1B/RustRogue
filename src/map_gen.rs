use crate::{
    create_player,
    map::{Map, TileType},
    Position, Renderable, RigidBody,
};
use rltk::RGB;
use specs::prelude::*;

pub fn get_glyph_tile(tile: &TileType) -> rltk::FontCharType {
    if *tile == TileType::Floor {
        rltk::to_cp437('.')
    } else {
        rltk::to_cp437('#')
    }
}

/// Given a vector of TileType, create all entities composing the map
pub fn gen_map_from_tile(map: &Map, ecs: &mut World) -> Vec<Entity> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut map_entity: Vec<Entity> = Vec::new();
    for tile in map.map.iter() {
        let entity = ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: get_glyph_tile(tile),
                fg: RGB::named(rltk::WHITE),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        map_entity.push(entity);
        let mut rigids = ecs.write_storage::<RigidBody>();
        if *tile == TileType::Wall {
            rigids
                .insert(entity, RigidBody {})
                .expect("Impossible to create a rigid body for entity in map_gen");
        }
        x = x + 1;
        if x == map.width {
            y = y + 1;
            x = 0;
        }
    }
    map_entity
}

/// Create a map and place a player inside
pub fn gen_map(height: i32, width: i32, ecs: &mut World) {
    let map = Map::new_map_rooms_and_corridors(width, height);
    let map_entity = gen_map_from_tile(&map, ecs);
    let (player_x, player_y) = map.rooms[0].center();
    create_player(player_x, player_y, ecs);
    ecs.insert(map_entity);
}
