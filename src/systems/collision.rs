use std::{borrow::BorrowMut, collections::HashSet};

use specs::prelude::*;

use crate::{Move, Position, RigidBody};

/// System which detect collision and cancel movement if possible
pub struct CollisionSystem {}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, RigidBody>,
        Entities<'a>,
        WriteStorage<'a, Move>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, rigids, entities, mut movs) = data;
        let mut to_delete: HashSet<Entity> = HashSet::new();
        for (pos, _rigid, entity1, mov1) in (&positions, &rigids, &entities, &movs).join() {
            for (pos2, _rigid2, entity2) in (&positions, &rigids, &entities).join() {
                if entity1 == entity2 {
                    continue;
                }
                let mov2 = movs.get(entity2);
                match mov2 {
                    None => {
                        if pos.x + mov1.move_x == pos2.x && pos2.y == pos.y + mov1.move_y {
                            to_delete.insert(entity1);
                        }
                    }
                    Some(mov2_r) => {
                        if pos2.x + mov2_r.move_x == pos.x + mov1.move_x
                            && pos.y + mov1.move_y == pos2.y + mov2_r.move_y
                        {
                            to_delete.insert(entity1);
                            to_delete.insert(entity2);
                        }
                    }
                }
            }
        }
        for entity in to_delete {
            movs.borrow_mut().remove(entity);
        }
    }
}
