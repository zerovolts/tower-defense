use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(audio_setup);
    }
}

type AudioHandle = Handle<AudioSource>;

pub struct AudioHandleMap {
    pub base_hit: AudioHandle,
    pub base_destroy: AudioHandle,
    pub enemy_destroy: AudioHandle,
    pub enemy_hit: AudioHandle,
    pub tower_place: AudioHandle,
    pub tower_shoot: AudioHandle,
}

fn audio_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioHandleMap {
        base_hit: asset_server.load("sfx/base_hit.flac"),
        base_destroy: asset_server.load("sfx/base_destroy.flac"),
        enemy_destroy: asset_server.load("sfx/enemy_destroy.flac"),
        enemy_hit: asset_server.load("sfx/enemy_hit.flac"),
        tower_place: asset_server.load("sfx/tower_place.flac"),
        tower_shoot: asset_server.load("sfx/tower_shoot.flac"),
    });
}
