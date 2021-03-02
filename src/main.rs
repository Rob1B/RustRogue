use std::borrow::BorrowMut;

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs::World;

pub mod components;
pub use components::*;
mod handler;
mod map;
mod map_gen;
mod render;
mod systems {
    pub mod collision;
    pub mod movement_system;
}

use crate::systems::collision::CollisionSystem;
use crate::systems::movement_system::MoveSystem;

const SHOW_FPS: bool = true;
struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut col = CollisionSystem {};
        col.run_now(&self.ecs);
        let mut mv = MoveSystem {};
        mv.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        handler::handle_keys(&self.ecs, ctx);
        self.run_systems();
        render::draw_entities(&self.ecs, ctx);
        if SHOW_FPS {
            ctx.print(1, 49, &format!("FPS: {}", ctx.fps));
        }
    }
}

fn create_player(x: i32, y: i32, ecs: &mut World) {
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Position { x, y })
        .with(Player {})
        .with(RigidBody {})
        .build();
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("RogueLike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Move>();
    gs.ecs.register::<RigidBody>();
    map_gen::gen_map(50, 80, gs.ecs.borrow_mut());
    rltk::main_loop(context, gs)
}
