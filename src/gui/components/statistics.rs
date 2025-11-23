use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Polygon};
use crate::gui::state::AppState;

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    use egui::{Layout, Align};
    let available = ui.available_size();
    
    ui.allocate_ui_with_layout(available, Layout::top_down(Align::Min), |ui| {
        ui.heading("Aggregated Results");
        
        let height = available.y / 2.0 - 20.0;

        // Distance Plot
        ui.label("Distance (Mean ± Std Dev)");
        Plot::new("Aggregated_Dist")
            .height(height)
            .show(ui, |plot_ui| {
                draw_aggregated_plot(state, plot_ui, |h| h.current_dist, egui::Color32::BLUE);
            });

        ui.separator();

        // Violation Plot
        ui.label("Violation (Mean ± Std Dev)");
        Plot::new("Aggregated_Viol")
            .height(height)
            .show(ui, |plot_ui| {
                draw_aggregated_plot(state, plot_ui, |h| h.current_viol, egui::Color32::RED);
            });
    });
}

pub fn draw_aggregated_plot(state: &AppState, plot_ui: &mut egui_plot::PlotUi, value_extractor: impl Fn(&crate::gui::state::LogEntry) -> f32, color: egui::Color32) {
    let max_iter = state.runs.iter().map(|r| r.history.len()).max().unwrap_or(0);
    if max_iter > 0 {
        let mut mean_points = Vec::new();
        let mut upper_points = Vec::new();
        let mut lower_points = Vec::new();
        
        // Sample every N iterations to save perf
        let step = (max_iter / 200).max(1);
        
        // Smoothing window size
        let window_size = 5;
        let mut history_buffer: Vec<f32> = Vec::new();

        for i in (0..max_iter).step_by(step) {
            let mut values = Vec::new();
            for run in &state.runs {
                if i < run.history.len() {
                    values.push(value_extractor(&run.history[i]));
                }
            }
            
            if !values.is_empty() {
                let sum: f32 = values.iter().sum();
                let count = values.len() as f32;
                let mean = sum / count;
                
                let variance: f32 = values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / count;
                let std_dev = variance.sqrt();
                
                // 95% Confidence Interval of the Mean: 1.96 * (std_dev / sqrt(N))
                let ci95 = 1.96 * (std_dev / count.sqrt());
                
                // Apply simple moving average smoothing
                history_buffer.push(mean);
                if history_buffer.len() > window_size {
                    history_buffer.remove(0);
                }
                let smoothed_mean = history_buffer.iter().sum::<f32>() / history_buffer.len() as f32;

                mean_points.push([i as f64, smoothed_mean as f64]);
                upper_points.push([i as f64, (smoothed_mean + ci95) as f64]);
                lower_points.push([i as f64, (smoothed_mean - ci95) as f64]);
            }
        }
        
        if !mean_points.is_empty() {
            let mut poly_points = upper_points;
            poly_points.extend(lower_points.into_iter().rev());
            
            let fill_color = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 50);
            plot_ui.polygon(Polygon::new(PlotPoints::new(poly_points)).fill_color(fill_color).name("95% CI"));
            plot_ui.line(Line::new(PlotPoints::new(mean_points)).color(color).name("Mean"));
        }
    }
}
