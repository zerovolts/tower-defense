use bevy::math::Vec2;

pub const CELL_SIZE: f32 = 32.0;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE * 0.5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

impl From<Coord> for Vec2 {
    fn from(coord: Coord) -> Self {
        Self::new(coord.x as f32 * CELL_SIZE, coord.y as f32 * CELL_SIZE)
    }
}
