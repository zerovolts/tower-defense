use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game_state::GameState;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency { coins: 10 })
            .add_enter_system(GameState::Playing, currency_setup)
            .add_system(currency_update.run_in_state(GameState::Playing));
    }
}

#[derive(Deref)]
pub struct Currency {
    pub coins: i32,
}

#[derive(Component)]
struct CoinsUi;

fn currency_setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    currency: Res<Currency>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                format!("Coins: {}", currency.coins),
                TextStyle {
                    font: asset_server.load("fonts/UbuntuMono-R.ttf"),
                    font_size: 24.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                },
                default(),
            ),
            ..default()
        })
        .insert(CoinsUi);
}

fn currency_update(currency: Res<Currency>, mut query: Query<&mut Text, With<CoinsUi>>) {
    if currency.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Coins: {}", currency.coins);
        }
    }
}
