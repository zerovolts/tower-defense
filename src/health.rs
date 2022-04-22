use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { max, current: max }
    }

    pub fn damage(&mut self, damage: i32) {
        if self.current > 0 {
            self.current -= damage;
        }
    }
}
