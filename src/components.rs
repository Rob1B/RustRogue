use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Move {
    pub move_x: i32,
    pub move_y: i32,
}

#[derive(Component)]
pub struct RigidBody {}
