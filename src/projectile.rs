use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    enemy::Enemy,
    health::Health,
    mesh::{MeshMaterial, RegPoly},
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectile>()
            .add_startup_system(projectile_setup)
            .add_system(apply_velocity)
            .add_system(projectile_spawn)
            .add_system(projectile_hit)
            .add_system(projectile_destroy);
    }
}

#[derive(Component)]
struct Projectile {
    creation_time: f64,
}

#[derive(Deref)]
struct ProjectileAssets(MeshMaterial);

fn projectile_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(ProjectileAssets(MeshMaterial {
        mesh: Mesh2dHandle(meshes.add(RegPoly::new(8, 2.0).into())),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
    }));
}

pub struct SpawnProjectile {
    pub position: Vec2,
    pub direction: Vec2,
}

fn projectile_spawn(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<ProjectileAssets>,
    mut events: EventReader<SpawnProjectile>,
) {
    for event in events.iter() {
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_translation(event.position.extend(0.0)),
                ..Default::default()
            })
            .insert(Projectile {
                creation_time: time.seconds_since_startup(),
            })
            .insert(Velocity(event.direction.normalize_or_zero() * 200.0));
    }
}

fn projectile_hit(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(&mut Health, &Transform), With<Enemy>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        for (mut enemy_health, enemy_transform) in enemy_query.iter_mut() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                < 20.0
            {
                enemy_health.damage(1);
                commands.entity(projectile_entity).despawn();
                // Projectiles should only affect a single enemy.
                break;
            }
        }
    }
}

fn projectile_destroy(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &Projectile)>,
) {
    for (entity, projectile) in query.iter() {
        if time.seconds_since_startup() - projectile.creation_time > 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct Velocity(Vec2);

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}
