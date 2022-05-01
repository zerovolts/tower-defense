use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};

use crate::{base::Base, currency::Currency, health::Health};

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
    mut egui_ctx: ResMut<EguiContext>,
    currency: Res<Currency>,
    base_query: Query<&Health, With<Base>>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Coins: {}", currency.coins));

            ui.separator();

            if let Ok(base_health) = base_query.get_single() {
                ui.label(format!(
                    "Health: {}/{}",
                    base_health.current, base_health.max
                ));
            }
        });
    });
}
