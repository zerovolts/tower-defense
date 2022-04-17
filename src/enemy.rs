use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    coord::{Coord, CELL_SIZE},
    currency::Currency,
    mesh::{MeshMaterial, RegPoly},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(enemy_setup)
            .add_system(enemy_spawn)
            .add_system(enemy_path_follow)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_destroy);
    }
}

#[derive(Component)]
pub struct Enemy;

fn enemy_destroy(
    mut commands: Commands,
    mut currency: ResMut<Currency>,
    query: Query<(Entity, &Health), (With<Enemy>, Changed<Health>)>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0 {
            currency.coins += 1;
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Deref)]
struct EnemyAssets(MeshMaterial);

fn enemy_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(Path::new(vec![
        Coord::new(-3, -3),
        Coord::new(-3, 3),
        Coord::new(3, 3),
        Coord::new(3, -3),
        Coord::new(-3, -3),
    ]));

    commands.insert_resource(EnemyAssets(MeshMaterial {
        mesh: Mesh2dHandle(meshes.add(RegPoly::new(4, 12.0).into())),
        material: materials.add(Color::rgb(1.0, 0.3, 0.0).into()),
    }));

    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(RegPoly::new(6, 14.0).into())),
            material: materials.add(Color::rgb(0.4, 0.2, 0.6).into()),
            transform: Transform::from_xyz(-96.0, -96.0, 1.0),
            ..Default::default()
        })
        .insert(EnemySpawner {
            last_spawn_time: 0.0,
        });
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
}

impl Health {
    fn new(max: i32) -> Self {
        Self { current: max }
    }
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

#[derive(Component)]
struct PathFollow {
    progress: f32,
}

fn enemy_path_follow(
    mut commands: Commands,
    time: Res<Time>,
    path: Res<Path>,
    mut query: Query<(Entity, &mut Transform, &mut PathFollow)>,
) {
    for (entity, mut transform, mut path_follow) in query.iter_mut() {
        path_follow.progress += 0.025 * time.delta_seconds();
        if path_follow.progress >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
        transform.translation = path.lerp(path_follow.progress).extend(0.0);
    }
}

#[derive(Component)]
struct EnemySpawner {
    last_spawn_time: f64,
}

fn enemy_spawn(
    mut commands: Commands,
    assets: Res<EnemyAssets>,
    time: Res<Time>,
    mut query: Query<(&mut EnemySpawner, &Transform)>,
) {
    for (mut spawner, transform) in query.iter_mut() {
        if time.seconds_since_startup() - spawner.last_spawn_time < 2.0 {
            continue;
        }

        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Health::new(6))
            .insert(PathFollow { progress: 0.0 });

        spawner.last_spawn_time = time.seconds_since_startup();
    }
}
