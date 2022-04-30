use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    base::BasePlugin, currency::CurrencyPlugin, enemy::EnemyPlugin, game_state::GameState,
    map::MapPlugin, projectile::ProjectilePlugin, tower::TowerPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .add_loopless_state(GameState::Playing)
            .add_plugin(EnemyPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(TowerPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(BasePlugin)
            .add_plugin(CurrencyPlugin)
            .add_startup_system(game_setup);
    }
}

fn game_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
