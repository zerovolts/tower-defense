use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

use std::f32::consts::{PI, TAU};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .init_resource::<ProjectileAssets>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tower_firing)
        .add_system(apply_velocity)
        .run();
}

#[derive(Default)]
struct ProjectileAssets {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Component, Default)]
struct Tower {
    last_projectile_time: f64,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut projectile_assets: ResMut<ProjectileAssets>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    *projectile_assets = ProjectileAssets {
        mesh: Mesh2dHandle(meshes.add(RegPoly::new(8, 2.0).into())),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
    };

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
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 12.0).into())),
            material: materials.add(Color::rgb(0.0, 0.5, 1.0).into()),
            transform: Transform::from_xyz(64.0, 64.0, 0.0),
            ..Default::default()
        })
        .insert(Tower::default())
        .with_children(|parent| {
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(24.0, 4.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(12.0, 0.0, 1.0),
                ..Default::default()
            });
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 12.0).into())),
            material: materials.add(Color::rgb(0.0, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-64.0, 64.0, 0.0),
            ..Default::default()
        })
        .insert(Tower::default())
        .with_children(|parent| {
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(24.0, 4.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(12.0, 0.0, 1.0),
                ..Default::default()
            });
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 12.0).into())),
            material: materials.add(Color::rgb(0.0, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-64.0, -64.0, 0.0),
            ..Default::default()
        })
        .insert(Tower::default())
        .with_children(|parent| {
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(24.0, 4.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(12.0, 0.0, 1.0),
                ..Default::default()
            });
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 12.0).into())),
            material: materials.add(Color::rgb(0.0, 0.5, 1.0).into()),
            transform: Transform::from_xyz(64.0, -64.0, 0.0),
            ..Default::default()
        })
        .insert(Tower::default())
        .with_children(|parent| {
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(24.0, 4.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(12.0, 0.0, 1.0),
                ..Default::default()
            });
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())),
                material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
        });

    // Enemy
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(4, 12.0).into())),
            material: materials.add(Color::rgb(1.0, 0.3, 0.0).into()),
            transform: Transform::from_xyz(-256.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Velocity(Vec2::new(32.0, 0.0)));
}

const CLOCKWISE: f32 = -1.0;
const COUNTER_CLOCKWISE: f32 = 1.0;
const ANGULAR_SPEED: f32 = TAU / 200.0;

fn tower_firing(
    mut commands: Commands,
    time: Res<Time>,
    projectile_assets: Res<ProjectileAssets>,
    mut tower_query: Query<(&mut Tower, &mut Transform), Without<Enemy>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let max_dist = 256.0;
    for (mut tower, mut tower_transform) in tower_query.iter_mut() {
        let mut closest_enemy_direction: Option<Vec2> = None;
        for enemy_transform in enemy_query.iter() {
            let dist_sq = tower_transform
                .translation
                .distance_squared(enemy_transform.translation);

            // Skip enemy if it's out of range.
            if dist_sq > max_dist * max_dist {
                continue;
            }

            // Skip enemy if it's not closer than the current closest enemy.
            if let Some(direction) = closest_enemy_direction {
                if dist_sq >= direction.length_squared() {
                    continue;
                }
            }

            // Assign new closest enemy
            closest_enemy_direction =
                Some((enemy_transform.translation - tower_transform.translation).truncate());
        }

        if let Some(target_direction) = closest_enemy_direction {
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
                    mesh: projectile_assets.mesh.clone(),
                    material: projectile_assets.material.clone(),
                    transform: tower_transform.clone(),
                    ..Default::default()
                })
                .insert(Projectile)
                .insert(Velocity(target_direction.normalize_or_zero() * 200.0));

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
