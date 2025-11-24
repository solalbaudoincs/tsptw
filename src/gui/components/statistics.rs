use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Polygon, Bar, BarChart};
use crate::gui::state::AppState;

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    use egui::{Layout, Align, Vec2};
    let available = ui.available_size();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.allocate_ui_with_layout(Vec2::new(available.x, available.y), Layout::top_down(Align::Min), |ui| {
            ui.heading("Aggregated Results");
            
            let plot_height = 250.0;

            // Distance Plot
            ui.label("Distance (Mean ± Std Dev)");
            Plot::new("Aggregated_Dist")
                .height(plot_height)
                .show(ui, |plot_ui| {
                    draw_aggregated_plot(state, plot_ui, |h| h.current_dist, egui::Color32::BLUE);
                });

            ui.separator();

            // Violation Plot
            ui.label("Violation (Mean ± Std Dev)");
            Plot::new("Aggregated_Viol")
                .height(plot_height)
                .show(ui, |plot_ui| {
                    draw_aggregated_plot(state, plot_ui, |h| h.current_viol, egui::Color32::RED);
                });

            // Dynamic Metric Plots
            if let Some(first_run) = state.runs.first() {
                for metric_name in &first_run.metric_names {
                    ui.separator();
                    ui.label(format!("{} (Mean ± Std Dev)", metric_name));
                    let name_clone = metric_name.clone();
                    Plot::new(format!("Aggregated_{}", metric_name))
                        .height(plot_height)
                        .show(ui, |plot_ui| {
                            draw_aggregated_plot(state, plot_ui, |h| *h.metrics.get(&name_clone).unwrap_or(&0.0), egui::Color32::GOLD);
                        });
                }
            }

            ui.add_space(20.0);
            ui.heading("Final Value Distributions");
            
            // Histograms
            ui.horizontal(|ui| {
                let width = (ui.available_width() / 2.0) - 10.0;
                ui.vertical(|ui| {
                    draw_histogram(ui, state, "Final Distance Distribution", |h| h.current_dist, egui::Color32::BLUE, width);
                });
                ui.vertical(|ui| {
                    draw_histogram(ui, state, "Final Violation Distribution", |h| h.current_viol, egui::Color32::RED, width);
                });
            });

            if let Some(first_run) = state.runs.first() {
                let mut metrics = first_run.metric_names.iter();
                while let Some(m1) = metrics.next() {
                    ui.horizontal(|ui| {
                        let width = (ui.available_width() / 2.0) - 10.0;
                        ui.vertical(|ui| {
                            let name_clone = m1.clone();
                            draw_histogram(ui, state, &format!("Final {} Distribution", m1), |h| *h.metrics.get(&name_clone).unwrap_or(&0.0), egui::Color32::GOLD, width);
                        });
                        
                        if let Some(m2) = metrics.next() {
                            ui.vertical(|ui| {
                                let name_clone = m2.clone();
                                draw_histogram(ui, state, &format!("Final {} Distribution", m2), |h| *h.metrics.get(&name_clone).unwrap_or(&0.0), egui::Color32::GOLD, width);
                            });
                        }
                    });
                }
            }
        });
    });
}

fn draw_histogram(ui: &mut egui::Ui, state: &AppState, title: &str, value_extractor: impl Fn(&crate::gui::state::LogEntry) -> f32, color: egui::Color32, width: f32) {
    let mut values: Vec<f32> = state.runs.iter()
        .filter_map(|r| r.history.last().map(|l| value_extractor(l)))
        .collect();
    
    if values.is_empty() { return; }
    
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let min = *values.first().unwrap();
    let max = *values.last().unwrap();
    let range = max - min;
    let bins = 15;
    let step = if range > 1e-6 { range / bins as f32 } else { 1.0 };
    
    let mut bars = Vec::new();
    // If range is effectively zero, just show one bar
    if range <= 1e-6 {
        bars.push(Bar::new(min as f64, values.len() as f64).width(0.5).fill(color));
    } else {
        for i in 0..bins {
            let start = min + i as f32 * step;
            let end = start + step;
            // Inclusive of end for the last bin
            let count = values.iter().filter(|&&v| v >= start && (v < end || (i == bins - 1 && v <= end))).count();
            
            if count > 0 {
                bars.push(Bar::new((start + step/2.0) as f64, count as f64).width(step as f64 * 0.9).fill(color));
            }
        }
    }
    
    ui.label(title);
    Plot::new(format!("Hist_{}", title))
        .width(width)
        .height(200.0)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(BarChart::new(bars).color(color));
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
            let mut values: Vec<f32> = Vec::new();
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
            let fill_color = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 50);
            
            // Draw CI as strips (quads) to avoid triangulation artifacts with complex/degenerate polygons
            for i in 0..mean_points.len().saturating_sub(1) {
                let p1_u = upper_points[i];
                let p2_u = upper_points[i+1];
                let p2_l = lower_points[i+1];
                let p1_l = lower_points[i];
                
                let quad = vec![p1_u, p2_u, p2_l, p1_l];
                plot_ui.polygon(Polygon::new(PlotPoints::new(quad)).fill_color(fill_color));
            }

            plot_ui.line(Line::new(PlotPoints::new(mean_points)).color(color).name("Mean"));
        }
    }
}
