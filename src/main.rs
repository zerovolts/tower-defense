use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::game::GamePlugin;

mod base;
mod coord;
mod currency;
mod enemy;
mod game;
mod game_state;
mod health;
mod map;
mod mesh;
mod projectile;
mod tower;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(GamePlugin)
        .run();
}
