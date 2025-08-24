use nannou::prelude::*;

#[derive(Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Rgb<u8>,
}

pub struct Model {
    pub balls: Vec<Ball>,
    pub dragging: Option<usize>,
    pub last_mouse_pos: Vec2,
    pub wells: Vec<Well>,
}

#[derive(Debug)]
pub struct Well {
    pub pos: Vec2,
    pub strength: f32,
}
