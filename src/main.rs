use bevy::prelude::*;

use crate::{enemy::EnemyPlugin, projectile::ProjectilePlugin, tower::TowerPlugin};

mod coord;
mod enemy;
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
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
