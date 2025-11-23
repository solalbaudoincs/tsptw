use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::gui::state::RunState;

pub fn show(ui: &mut egui::Ui, run_state: &RunState, log_scale: &mut bool) {
    ui.push_id("metrics_plot", |ui| {
        let available_height = ui.available_height();
        let num_plots = 2 + run_state.metric_names.len(); // Distance + Violation + metrics
        let plot_height = (available_height - (num_plots as f32 * 20.0)) / num_plots as f32;

        // Distance Plot
        ui.label("Distance");
        Plot::new("Distance")
            .height(plot_height)
            .show(ui, |plot_ui| {
                let fitness_points: PlotPoints = run_state.history.iter()
                    .map(|e| [e.iteration as f64, e.current_dist as f64])
                    .collect();
                plot_ui.line(Line::new(fitness_points).name("Distance"));
            });

        // Violation Plot
        ui.horizontal(|ui| {
            ui.label("Violation");
            ui.checkbox(log_scale, "Log Scale (ln(x+1))");
        });
        
        Plot::new("Violation")
            .height(plot_height)
            .show(ui, |plot_ui| {
                let use_log = *log_scale;
                let points: PlotPoints = run_state.history.iter()
                    .map(|e| {
                        let val = e.current_viol as f64;
                        if use_log {
                            [e.iteration as f64, (val + 1.0).ln()]
                        } else {
                            [e.iteration as f64, val]
                        }
                    })
                    .collect();
                plot_ui.line(Line::new(points).name("Violation").color(egui::Color32::RED));
            });

        // Metric Plots
        for name in &run_state.metric_names {
             ui.label(name);
             Plot::new(name)
                .height(plot_height)
                .show(ui, |plot_ui| {
                    let points: PlotPoints = run_state.history.iter()
                        .filter_map(|e| e.metrics.get(name).map(|v| [e.iteration as f64, *v as f64]))
                        .collect();
                    plot_ui.line(Line::new(points).name(name));
                });
        }
    });
}


