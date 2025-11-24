use eframe::egui;
use crate::gui::state::{AppState, ViewMode, AppPhase};
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

        match self.state.phase {
            AppPhase::Configuration => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    components::welcome::show(ui, &mut self.state);
                });
            },
            AppPhase::Running => {
                // Top bar for navigation/control could go here if needed
                egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Back to Config").clicked() {
                            self.state.phase = AppPhase::Configuration;
                            self.state.runs.clear();
                            self.state.selected_run_index = None;
                        }
                        ui.separator();
                        ui.selectable_value(&mut self.state.view_mode, ViewMode::Grid, "Grid View");
                        ui.selectable_value(&mut self.state.view_mode, ViewMode::Statistics, "Statistics View");
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(idx) = self.state.selected_run_index {
                        components::dashboard::show(ui, &mut self.state, idx);
                    } else {
                        match self.state.view_mode {
                            ViewMode::Grid => components::grid::show(ui, &mut self.state),
                            ViewMode::Statistics => components::statistics::show(ui, &self.state),
                        }
                    }
                });
            }
        }
    }
}

