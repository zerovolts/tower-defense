use bevy::prelude::*;

use crate::{
    base::BasePlugin, currency::CurrencyPlugin, enemy::EnemyPlugin, map::MapPlugin,
    projectile::ProjectilePlugin, tower::TowerPlugin,
};

mod base;
mod coord;
mod currency;
mod enemy;
mod health;
mod map;
mod mesh;
mod projectile;
mod tower;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EnemyPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(BasePlugin)
        .add_plugin(CurrencyPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
