use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    sprite::Mesh2dHandle,
};

use std::f32::consts::{PI, TAU};

use crate::{
    coord::{Coord, CELL_SIZE, HALF_CELL_SIZE},
    currency::Currency,
    enemy::Enemy,
    mesh::{MeshMaterial, RegPoly},
    projectile::SpawnProjectile,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTower>()
            .add_event::<SpawnBuildSpot>()
            .add_startup_system(tower_setup)
            .add_system(tower_place)
            .add_system(tower_spawn)
            .add_system(tower_shoot)
            .add_system(build_spot_spawn);
    }
}

#[derive(Component, Default)]
struct Tower {
    target: Option<Entity>,
    last_projectile_time: f64,
}

#[derive(Component, Deref)]
pub struct GridPosition(Coord);

#[derive(Default)]
struct TowerAssets {
    base: MeshMaterial,
    barrel: MeshMaterial,
    barrel_cap: MeshMaterial,
}

fn tower_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut build_spot_spawn_events: EventWriter<SpawnBuildSpot>,
) {
    commands.insert_resource(TowerAssets {
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
    });

    commands.insert_resource(BuildSpotAssets(MeshMaterial {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    }));

    for x in -1..=1 {
        for y in -1..=1 {
            build_spot_spawn_events.send(SpawnBuildSpot {
                position: Coord::new(x, y),
            });
        }
    }
}

struct SpawnTower {
    position: Coord,
}

fn tower_spawn(
    mut commands: Commands,
    assets: Res<TowerAssets>,
    mut events: EventReader<SpawnTower>,
) {
    for event in events.iter() {
        let position: Vec2 = event.position.into();
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.base.mesh.clone(),
                material: assets.base.material.clone(),
                transform: Transform::from_translation(position.extend(1.0)),
                ..Default::default()
            })
            .insert(Tower::default())
            .insert(GridPosition(event.position))
            .with_children(|parent| {
                parent.spawn_bundle(ColorMesh2dBundle {
                    mesh: assets.barrel.mesh.clone(),
                    material: assets.barrel.material.clone(),
                    transform: Transform::from_xyz(12.0, 0.0, 2.0),
                    ..Default::default()
                });
                parent.spawn_bundle(ColorMesh2dBundle {
                    mesh: assets.barrel_cap.mesh.clone(),
                    material: assets.barrel_cap.material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    ..Default::default()
                });
            });
    }
}

const CLOCKWISE: f32 = -1.0;
const COUNTER_CLOCKWISE: f32 = 1.0;
const ANGULAR_SPEED: f32 = TAU / 200.0;
const MAX_DISTANCE: f32 = 256.0;

fn tower_shoot(
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
struct BuildSpot;

#[derive(Deref)]
struct BuildSpotAssets(MeshMaterial);

struct SpawnBuildSpot {
    position: Coord,
}

fn build_spot_spawn(
    mut commands: Commands,
    mut events: EventReader<SpawnBuildSpot>,
    assets: Res<BuildSpotAssets>,
) {
    for event in events.iter() {
        let v: Vec2 = event.position.into();
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_translation(v.extend(0.0)),
                ..Default::default()
            })
            .insert(BuildSpot)
            .insert(GridPosition(event.position));
    }
}

fn tower_place(
    mut currency: ResMut<Currency>,
    mut tower_spawn_events: EventWriter<SpawnTower>,
    mut mouse_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    build_spot_query: Query<&GridPosition, With<BuildSpot>>,
    tower_query: Query<&GridPosition, With<Tower>>,
) {
    let window = windows.get_primary().expect("No primary window");
    for mouse_event in mouse_events.iter() {
        if let Some(position) = cursor_coord(&window) {
            if mouse_event.button == MouseButton::Left
                && mouse_event.state == ElementState::Pressed
                && currency.coins >= 5
                && build_spot_query
                    .iter()
                    .any(|build_spot_position| build_spot_position.0 == position)
                && !tower_query
                    .iter()
                    .any(|tower_position| tower_position.0 == position)
            {
                currency.coins -= 5;
                tower_spawn_events.send(SpawnTower { position });
            }
        }
    }
}

fn cursor_coord(window: &Window) -> Option<Coord> {
    if let Some(position) = window.cursor_position() {
        let half_width = window.width() * 0.5;
        let half_height = window.height() * 0.5;
        Some(Coord::new(
            ((position.x as f32 - half_width + HALF_CELL_SIZE) / CELL_SIZE).floor() as i32,
            ((position.y as f32 - half_height + HALF_CELL_SIZE) / CELL_SIZE).floor() as i32,
        ))
    } else {
        None
    }
}
