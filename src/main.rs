use bevy::{prelude::*, sprite::Mesh2dHandle};

use std::f32::consts::{PI, TAU};

use crate::{
    coord::Coord,
    enemy::{Enemy, EnemyPlugin, Health},
    mesh::{MeshMaterial, RegPoly},
};

mod coord;
mod enemy;
mod mesh;

fn main() {
    App::new()
        .add_plugin(EnemyPlugin)
        .add_event::<SpawnTower>()
        .add_event::<SpawnProjectile>()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .init_resource::<ArtAssets>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tower_firing)
        .add_system(apply_velocity)
        .add_system(spawn_projectile)
        .add_system(spawn_towers)
        .add_system(destroy_projectile)
        .add_system(projectile_hit)
        .run();
}

struct SpawnTower {
    position: Coord,
}

#[derive(Default)]
struct ArtAssets {
    projectile: MeshMaterial,
    tower: TowerAssets,
}

#[derive(Default)]
struct TowerAssets {
    base: MeshMaterial,
    barrel: MeshMaterial,
    barrel_cap: MeshMaterial,
}
#[derive(Component, Default)]
struct Tower {
    target: Option<Entity>,
    last_projectile_time: f64,
}

#[derive(Component)]
struct Projectile {
    creation_time: f64,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut art_assets: ResMut<ArtAssets>,
    mut tower_spawn_events: EventWriter<SpawnTower>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    *art_assets = ArtAssets {
        projectile: MeshMaterial {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(8, 2.0).into())),
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        },
        tower: TowerAssets {
            base: MeshMaterial {
                mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 12.0).into())),
                material: materials.add(Color::rgb(0.0, 0.5, 1.0).into()),
            },
            barrel: MeshMaterial {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(24.0, 4.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            },
            barrel_cap: MeshMaterial {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            },
        },
    };

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

    // Tower
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(1, 1),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(-1, 1),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(-1, -1),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(1, -1),
    });
}

fn spawn_towers(
    mut commands: Commands,
    assets: Res<ArtAssets>,
    mut events: EventReader<SpawnTower>,
) {
    for event in events.iter() {
        let position: Vec2 = event.position.into();
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.tower.base.mesh.clone(),
                material: assets.tower.base.material.clone(),
                transform: Transform::from_translation(position.extend(0.0)),
                ..Default::default()
            })
            .insert(Tower::default())
            .with_children(|parent| {
                parent.spawn_bundle(ColorMesh2dBundle {
                    mesh: assets.tower.barrel.mesh.clone(),
                    material: assets.tower.barrel.material.clone(),
                    transform: Transform::from_xyz(12.0, 0.0, 1.0),
                    ..Default::default()
                });
                parent.spawn_bundle(ColorMesh2dBundle {
                    mesh: assets.tower.barrel_cap.mesh.clone(),
                    material: assets.tower.barrel_cap.material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..Default::default()
                });
            });
    }
}

const CLOCKWISE: f32 = -1.0;
const COUNTER_CLOCKWISE: f32 = 1.0;
const ANGULAR_SPEED: f32 = TAU / 200.0;
const MAX_DISTANCE: f32 = 256.0;

fn tower_firing(
    time: Res<Time>,
    mut events: EventWriter<SpawnProjectile>,
    mut tower_query: Query<(&mut Tower, &mut Transform), Without<Enemy>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (mut tower, mut tower_transform) in tower_query.iter_mut() {
        // Loop hack: the loop here is only to early-return from the block, not
        // to actually loop.
        let target_direction = loop {
            // Check whether the current target is still in range.
            if let Some(target) = tower.target {
                let enemy = enemy_query.get(target);
                if let Ok((_, enemy_transform)) = enemy {
                    let dist_sq = tower_transform
                        .translation
                        .distance_squared(enemy_transform.translation);

                    if dist_sq <= MAX_DISTANCE * MAX_DISTANCE {
                        break Some(
                            (enemy_transform.translation - tower_transform.translation).truncate(),
                        );
                    }
                }
            };

            // Search for a new target.
            let closest_enemy_direction: Option<Vec2> =
                enemy_query
                    .iter()
                    .fold(None, |closest, (enemy, enemy_transform)| {
                        let dist_sq = tower_transform
                            .translation
                            .distance_squared(enemy_transform.translation);

                        // Skip enemy if it's out of range.
                        if dist_sq > MAX_DISTANCE * MAX_DISTANCE {
                            return closest;
                        }

                        // Skip enemy if it's not closer than the current closest enemy.
                        if let Some(direction) = closest {
                            if dist_sq >= direction.length_squared() {
                                return closest;
                            }
                        }

                        tower.target = Some(enemy);
                        return Some(
                            (enemy_transform.translation - tower_transform.translation).truncate(),
                        );
                    });

            if closest_enemy_direction.is_none() {
                tower.target = None;
            }
            break closest_enemy_direction;
        };

        if let Some(target_direction) = target_direction {
            let target_angle = target_direction.into_angle();
            let current_angle = {
                let (axis, angle) = tower_transform.rotation.to_axis_angle();
                normalize_angle(angle * axis.z)
            };
            let angle_to_target = target_angle - current_angle;

            if angle_to_target.abs() > ANGULAR_SPEED {
                let spin = if target_angle > current_angle {
                    if angle_to_target < PI {
                        COUNTER_CLOCKWISE
                    } else {
                        CLOCKWISE
                    }
                } else {
                    if angle_to_target > -PI {
                        CLOCKWISE
                    } else {
                        COUNTER_CLOCKWISE
                    }
                };
                tower_transform.rotate(Quat::from_rotation_z(ANGULAR_SPEED * spin));
                continue;
            }
            tower_transform.rotation = Quat::from_rotation_z(target_angle);

            if !(tower.last_projectile_time + 1.0 < time.seconds_since_startup()) {
                continue;
            }

            events.send(SpawnProjectile {
                position: tower_transform.translation.truncate(),
                direction: target_direction,
            });

            tower.last_projectile_time = time.seconds_since_startup();
        }
    }
}

struct SpawnProjectile {
    position: Vec2,
    direction: Vec2,
}

fn spawn_projectile(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<ArtAssets>,
    mut events: EventReader<SpawnProjectile>,
) {
    for event in events.iter() {
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.projectile.mesh.clone(),
                material: assets.projectile.material.clone(),
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
                if enemy_health.current > 0 {
                    enemy_health.current -= 1;
                }
                commands.entity(projectile_entity).despawn();
                // Projectiles should only affect a single enemy.
                break;
            }
        }
    }
}

fn destroy_projectile(
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

fn normalize_angle(angle: f32) -> f32 {
    if angle < 0.0 {
        return normalize_angle(angle + TAU);
    }
    if angle >= TAU {
        return normalize_angle(angle - TAU);
    }
    angle
}

trait IntoAngle {
    fn into_angle(self) -> f32;
}

impl IntoAngle for Vec2 {
    fn into_angle(self) -> f32 {
        let angle = f32::atan2(self.y, self.x);
        if angle < 0.0 {
            angle + TAU
        } else {
            angle
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
