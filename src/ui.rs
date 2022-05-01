use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};
use iyes_loopless::prelude::*;

use crate::{
    base::Base, currency::Currency, enemy::PlayTime, game_state::GameState, health::Health,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup).add_system(ui);
    }
}

fn ui_setup(mut egui_ctx: ResMut<EguiContext>, mut egui_settings: ResMut<EguiSettings>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        dark_mode: false,
        window_rounding: 0.0.into(),
        ..default()
    });

    egui_settings.scale_factor = 1.5;
}

fn ui(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    currency: Res<Currency>,
    play_time: Res<PlayTime>,
    game_state: Res<CurrentState<GameState>>,
    base_query: Query<&Health, With<Base>>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(24.0);

            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.label(format!("Coins: {}", currency.coins));

                ui.separator();

                if let Ok(base_health) = base_query.get_single() {
                    ui.label(format!(
                        "Health: {}/{}",
                        base_health.current, base_health.max
                    ));
                }
            });

            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                let seconds = play_time.seconds.floor() as i32;
                let clock = format!(
                    "{:02}:{:02}:{:02}",
                    seconds / 3600,
                    seconds / 60,
                    seconds % 60
                );
                if game_state.0 == GameState::Paused {
                    ui.scope(|ui| {
                        ui.visuals_mut().override_text_color =
                            Some(egui::Color32::from_rgb(255, 95, 0));
                        ui.label(clock);
                    });
                } else {
                    ui.label(clock);
                }

                if game_state.0 == GameState::Playing {
                    if ui.add(egui::widgets::Button::new("⏸")).clicked() {
                        commands.insert_resource(NextState(GameState::Paused));
                    }
                } else if game_state.0 == GameState::Paused {
                    if ui.add(egui::widgets::Button::new("▶")).clicked() {
                        commands.insert_resource(NextState(GameState::Playing));
                    }
                }
            });
        });
    });
}
