use eframe::egui;
use crate::gui::state::{AppState, ViewMode};
use crate::gui::config;
use crate::gui::components;

pub struct TspApp {
    state: AppState,
}

impl TspApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(),
        }
    }
}

impl eframe::App for TspApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Logic
        self.state.update_solvers();
        if self.state.runs.iter().any(|r| r.is_running) {
            ctx.request_repaint();
        }

        // UI
        egui::SidePanel::left("config_panel").show(ctx, |ui| {
            config::show(ui, &mut self.state);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.state.selected_run_index {
                components::dashboard::show(ui, &mut self.state, idx);
            } else {
                // Main View (Grid or Statistics)
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.state.view_mode, ViewMode::Grid, "Grid View");
                    ui.selectable_value(&mut self.state.view_mode, ViewMode::Statistics, "Statistics View");
                });
                ui.separator();

                match self.state.view_mode {
                    ViewMode::Grid => components::grid::show(ui, &mut self.state),
                    ViewMode::Statistics => components::statistics::show(ui, &self.state),
                }
            }
        });
    }
}

