use rltk::Rltk;
use specs::prelude::*;

use crate::{Position, Renderable};

/// Function drawing all entities on screen
pub fn draw_entities(ecs: &World, _ctx: &mut Rltk) {
    let positions = ecs.read_storage::<Position>();
    let render = ecs.read_storage::<Renderable>();
    for (pos, render) in (&positions, &render).join() {
        _ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}
