use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

use std::f32::consts::{PI, TAU};

fn main() {
    App::new()
        .add_event::<EnemyDestroyed>()
        .add_event::<SpawnTower>()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .init_resource::<ArtAssets>()
        .init_resource::<Path>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tower_firing)
        .add_system(apply_velocity)
        .add_system(follow_path)
        .add_system(spawn_enemies)
        .add_system(spawn_towers)
        .add_system(destroy_projectile)
        .add_system(projectile_hit)
        .add_system_to_stage(CoreStage::PostUpdate, destroy_enemy)
        .run();
}

struct EnemyDestroyed {
    enemy: Entity,
}

struct SpawnTower {
    position: Coord,
}

const CELL_SIZE: f32 = 32.0;

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

impl From<Coord> for Vec2 {
    fn from(coord: Coord) -> Self {
        Self::new(coord.x as f32 * CELL_SIZE, coord.y as f32 * CELL_SIZE)
    }
}

#[derive(Default)]
struct ArtAssets {
    projectile: MeshMaterial,
    enemy: MeshMaterial,
    tower: TowerAssets,
}

#[derive(Default)]
struct TowerAssets {
    base: MeshMaterial,
    barrel: MeshMaterial,
    barrel_cap: MeshMaterial,
}

#[derive(Default)]
struct MeshMaterial {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Component, Default)]
struct Tower {
    target: Option<Entity>,
    last_projectile_time: f64,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile {
    creation_time: f64,
}

#[derive(Component)]
struct PathFollow {
    progress: f32,
}

#[derive(Default)]
struct Path {
    nodes: Vec<Coord>,
    segment_lengths: Vec<i32>,
}

impl Path {
    fn new(nodes: Vec<Coord>) -> Path {
        let segment_lengths = nodes
            .windows(2)
            .map(|segment| {
                (segment[1].x - segment[0].x).abs() + (segment[1].y - segment[0].y).abs()
            })
            .collect();

        Self {
            nodes,
            segment_lengths,
        }
    }

    fn lerp(&self, progress: f32) -> Vec2 {
        let mut tile_progress = (self.length() as f32 * CELL_SIZE) * progress;
        for (i, segment) in self.nodes.windows(2).enumerate() {
            let segment_length = self.segment_lengths[i] as f32 * CELL_SIZE;
            if tile_progress > segment_length {
                tile_progress -= segment_length;
                continue;
            }
            let segment_progress = tile_progress / segment_length;
            let segment_start: Vec2 = segment[0].into();
            let segment_relative_end: Vec2 =
                Coord::new(segment[1].x - segment[0].x, segment[1].y - segment[0].y).into();
            return segment_start + segment_relative_end * segment_progress;
        }
        // Progress is outside of 0.0 - 1.0
        Vec2::default()
    }

    fn length(&self) -> i32 {
        self.segment_lengths.iter().fold(0, |acc, cur| acc + cur)
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut art_assets: ResMut<ArtAssets>,
    mut path: ResMut<Path>,
    mut tower_spawn_events: EventWriter<SpawnTower>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    *art_assets = ArtAssets {
        projectile: MeshMaterial {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(8, 2.0).into())),
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        },
        enemy: MeshMaterial {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(4, 12.0).into())),
            material: materials.add(Color::rgb(1.0, 0.3, 0.0).into()),
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

    *path = Path::new(vec![
        Coord::new(-6, -6),
        Coord::new(-6, 6),
        Coord::new(6, 6),
        Coord::new(6, -6),
    ]);

    // Build Slots
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(64.0, 64.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(-64.0, 64.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(-64.0, -64.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(30.0, 30.0)).into())),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(64.0, -64.0, 0.0),
        ..Default::default()
    });

    // Tower
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(2, 2),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(-2, 2),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(-2, -2),
    });
    tower_spawn_events.send(SpawnTower {
        position: Coord::new(2, -2),
    });

    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 14.0).into())),
            material: materials.add(Color::rgb(0.4, 0.2, 0.6).into()),
            transform: Transform::from_xyz(-192.0, -192.0, 1.0),
            ..Default::default()
        })
        .insert(EnemySpawner {
            last_spawn_time: 0.0,
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

#[derive(Component)]
struct EnemySpawner {
    last_spawn_time: f64,
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<ArtAssets>,
    time: Res<Time>,
    mut query: Query<(&mut EnemySpawner, &Transform)>,
) {
    for (mut spawner, transform) in query.iter_mut() {
        if time.seconds_since_startup() - spawner.last_spawn_time < 1.0 {
            continue;
        }

        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.enemy.mesh.clone(),
                material: assets.enemy.material.clone(),
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(PathFollow { progress: 0.0 });

        spawner.last_spawn_time = time.seconds_since_startup();
    }
}

const CLOCKWISE: f32 = -1.0;
const COUNTER_CLOCKWISE: f32 = 1.0;
const ANGULAR_SPEED: f32 = TAU / 200.0;
const MAX_DISTANCE: f32 = 256.0;

fn tower_firing(
    mut commands: Commands,
    time: Res<Time>,
    art_assets: Res<ArtAssets>,
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

            commands
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: art_assets.projectile.mesh.clone(),
                    material: art_assets.projectile.material.clone(),
                    transform: tower_transform.clone(),
                    ..Default::default()
                })
                .insert(Projectile {
                    creation_time: time.seconds_since_startup(),
                })
                .insert(Velocity(target_direction.normalize_or_zero() * 200.0));

            tower.last_projectile_time = time.seconds_since_startup();
        }
    }
}

fn follow_path(
    time: Res<Time>,
    path: Res<Path>,
    mut events: EventWriter<EnemyDestroyed>,
    mut query: Query<(Entity, &mut Transform, &mut PathFollow)>,
) {
    for (entity, mut transform, mut path_follow) in query.iter_mut() {
        path_follow.progress += 0.1 * time.delta_seconds();
        if path_follow.progress >= 1.0 {
            events.send(EnemyDestroyed { enemy: entity })
        }
        transform.translation = path.lerp(path_follow.progress).extend(0.0);
    }
}

fn projectile_hit(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for enemy_transform in enemy_query.iter() {
        for (projectile_entity, projectile_transform) in projectile_query.iter() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                < 20.0
            {
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

fn destroy_enemy(mut commands: Commands, mut events: EventReader<EnemyDestroyed>) {
    for event in events.iter() {
        commands.entity(event.enemy).despawn();
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

#[derive(Clone, Copy)]
struct RegPoly {
    pub sides: u32,
    pub radius: f32,
}

impl RegPoly {
    fn new(sides: u32, radius: f32) -> Self {
        Self { sides, radius }
    }
}

impl From<RegPoly> for Mesh {
    fn from(polygon: RegPoly) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = Vec::with_capacity(polygon.sides as usize + 1);
        positions.push([0.0, 0.0, 0.0]);
        for i in 0..polygon.sides {
            let angle = (i as f32) / (polygon.sides as f32) * TAU;
            positions.push([
                angle.cos() * polygon.radius,
                angle.sin() * polygon.radius,
                0.0,
            ]);
        }

        let normals = vec![[0.0, 0.0, 1.0]; (polygon.sides + 1) as usize];

        let uvs = positions
            .iter()
            .map(|v| {
                [
                    v[0] / polygon.radius / 2.0 + 0.5,
                    v[1] / polygon.radius / 2.0 + 0.5,
                ]
            })
            .collect::<Vec<[f32; 2]>>();

        let mut indices = Vec::with_capacity(polygon.sides as usize * 3);
        for i in 1..(polygon.sides) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }
        indices.push(0);
        indices.push(polygon.sides);
        indices.push(1);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
