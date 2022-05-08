use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;

use crate::game::GamePlugin;

mod audio;
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
        .add_plugin(AudioPlugin)
        .add_plugin(GamePlugin)
        .run();
}
