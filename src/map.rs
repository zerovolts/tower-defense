use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    base::SpawnBase,
    coord::Coord,
    enemy::{Path, SpawnEnemySpawner},
    game_state::GameState,
    tower::SpawnBuildSpot,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, map_setup);
    }
}

struct Map<'a> {
    path: &'a [Coord],
    build_spots: &'a [Coord],
}

const MAP: Map = Map {
    path: &[
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(1, -2),
        Coord::new(-2, -2),
        Coord::new(-2, 2),
        Coord::new(3, 2),
        Coord::new(3, -4),
        Coord::new(-2, -4),
    ],
    build_spots: &[
        Coord::new(0, -1),
        Coord::new(-1, -1),
        Coord::new(-1, 0),
        Coord::new(-1, 1),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(2, 0),
        Coord::new(2, -1),
        Coord::new(2, -2),
        Coord::new(2, -3),
        Coord::new(1, -3),
        Coord::new(0, -3),
        Coord::new(-1, -3),
        Coord::new(-2, -3),
    ],
};

pub fn map_setup(
    mut commands: Commands,
    mut build_spot_spawn_events: EventWriter<SpawnBuildSpot>,
    mut enemy_spawner_spawn_events: EventWriter<SpawnEnemySpawner>,
    mut base_spawn_events: EventWriter<SpawnBase>,
) {
    enemy_spawner_spawn_events.send(SpawnEnemySpawner {
        position: MAP.path[0],
    });

    base_spawn_events.send(SpawnBase {
        position: *MAP
            .path
            .last()
            .expect("Path must have at least one segment"),
    });

    commands.insert_resource(Path::new(Vec::from(MAP.path)));

    for &position in MAP.build_spots {
        build_spot_spawn_events.send(SpawnBuildSpot { position });
    }
}
