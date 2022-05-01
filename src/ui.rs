use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};

use crate::currency::Currency;

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

fn ui(mut egui_ctx: ResMut<EguiContext>, currency: Res<Currency>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Coins: {}", currency.coins));
        });
    });
}
