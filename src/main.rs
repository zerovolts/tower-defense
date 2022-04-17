use bevy::{prelude::*, sprite::Mesh2dHandle};

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

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Build Slots
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(32.0, 32.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(-32.0, 32.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(-32.0, -32.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(32.0, -32.0, 0.0),
        ..Default::default()
    });
}
