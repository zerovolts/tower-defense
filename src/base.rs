use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_kira_audio::Audio;
use iyes_loopless::prelude::*;

use crate::{
    audio::AudioHandleMap,
    coord::Coord,
    game_state::GameState,
    health::Health,
    mesh::{MeshMaterial, RegPoly},
};

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBase>()
            .add_startup_system(base_setup)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(base_spawn)
                    .with_system(base_destroy)
                    .into(),
            );
    }
}

#[derive(Component)]
pub struct Base;

#[derive(Deref)]
struct BaseAssets(MeshMaterial);

fn base_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(BaseAssets(MeshMaterial {
        mesh: Mesh2dHandle(meshes.add(RegPoly::fill(6, 12.0).into())),
        material: materials.add(Color::rgb(6.0, 0.6, 0.2).into()),
    }));
}

pub struct SpawnBase {
    pub position: Coord,
}

fn base_spawn(mut commands: Commands, assets: Res<BaseAssets>, mut events: EventReader<SpawnBase>) {
    for event in events.iter() {
        let position: Vec2 = event.position.into();
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_translation(position.extend(1.0)),
                ..Default::default()
            })
            .insert(Health::new(20))
            .insert(Base);
    }
}

fn base_destroy(
    mut commands: Commands,
    sounds: Res<AudioHandleMap>,
    audio: Res<Audio>,
    query: Query<&Health, (With<Base>, Changed<Health>)>,
) {
    for health in query.iter() {
        if health.current <= 0 {
            commands.insert_resource(NextState(GameState::GameOver));
            audio.play(sounds.base_destroy.clone());
        }
    }
}
