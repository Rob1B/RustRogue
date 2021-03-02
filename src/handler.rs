use crate::{Move, Player};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

/// Handle keys to enable movement
pub fn handle_keys(ecs: &World, _ctx: &mut Rltk) {
    let entities = ecs.entities();
    let player = ecs.read_storage::<Player>();
    let mut mov = ecs.write_storage::<Move>();
    for (entity, _player) in (&entities, &player).join() {
        match _ctx.key {
            None => {}
            Some(key) => {
                match key {
                    VirtualKeyCode::Up => mov
                        .insert(
                            entity,
                            Move {
                                move_x: 0,
                                move_y: -1,
                            },
                        )
                        .expect("Impossible"),
                    VirtualKeyCode::Down => mov
                        .insert(
                            entity,
                            Move {
                                move_x: 0,
                                move_y: 1,
                            },
                        )
                        .expect("Impossible"),
                    VirtualKeyCode::Left => mov
                        .insert(
                            entity,
                            Move {
                                move_x: -1,
                                move_y: 0,
                            },
                        )
                        .expect("Impossible"),
                    VirtualKeyCode::Right => mov
                        .insert(
                            entity,
                            Move {
                                move_x: 1,
                                move_y: 0,
                            },
                        )
                        .expect("Impossible"),
                    _ => None,
                };
            }
        }
    }
}
