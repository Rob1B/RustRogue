use crate::{Move, Position};
use specs::prelude::*;

/// System enabling movement of an Entity
pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Move>,
        WriteStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut to_delete: Vec<Entity> = Vec::new();
        let (mut mov, mut pos, entities) = data;
        for (movement, mut pos, entity) in (&mut mov, &mut pos, &entities).join() {
            pos.x += movement.move_x;
            pos.y += movement.move_y;
            to_delete.push(entity);
        }
        for entity in to_delete {
            mov.remove(entity);
        }
    }
}
