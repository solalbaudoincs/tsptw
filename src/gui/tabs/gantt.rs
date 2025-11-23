use eframe::egui;
use egui_plot::{Bar, BarChart, Legend, Orientation, Plot, PlotPoint, PlotPoints, Points, Text};
use crate::gui::state::RunState;
use crate::problem::Instance;

pub fn show(ui: &mut egui::Ui, run_state: &RunState, instance: &Option<Instance>) {
    ui.push_id("gantt_plot", |ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Gantt Chart Visualization").strong());
            ui.label("• X-Axis: Time units");
            ui.label("• Y-Axis: Node Index");
            ui.horizontal(|ui| {
                ui.label("Legend:");
                ui.colored_label(egui::Color32::GRAY, "■ Time Window");
                ui.colored_label(egui::Color32::YELLOW, "■ Wait Time");
                ui.colored_label(egui::Color32::GREEN, "● On Time");
                ui.colored_label(egui::Color32::RED, "● Late (Violation)");
            });
        });
        
        Plot::new("Gantt Schedule")
            .legend(Legend::default())
            .x_axis_label("Time")
            .y_axis_label("Node Index")
            .show(ui, |plot_ui| {
                if let Some(instance) = instance {
                    let schedule = run_state.get_schedule(instance);
                    
                    // Time Windows
                    let mut bars = Vec::new();
                    for (_i, visit) in schedule.iter().enumerate() {
                        let mut bar = Bar::new(visit.node_idx as f64, (visit.window_end - visit.window_start) as f64)
                            .base_offset(visit.window_start as f64)
                            .fill(egui::Color32::from_gray(200).linear_multiply(0.5))
                            .name("Time Window");
                        bar.orientation = Orientation::Horizontal;
                        bars.push(bar);
                    }
                    plot_ui.bar_chart(BarChart::new(bars).name("Time Windows").color(egui::Color32::GRAY));

                    // Arrivals
                    let mut arrival_points = Vec::new();
                    let mut violation_points = Vec::new();
                    
                    for (i, visit) in schedule.iter().enumerate() {
                        if visit.violation > 0.0 {
                            violation_points.push([visit.arrival_time as f64, visit.node_idx as f64]);
                        } else {
                            arrival_points.push([visit.arrival_time as f64, visit.node_idx as f64]);
                        }
                        
                        // Add visit order label
                        plot_ui.text(Text::new(
                            PlotPoint::new(visit.arrival_time as f64, visit.node_idx as f64 + 0.3), 
                            format!("#{}", i)
                        ).color(egui::Color32::WHITE));
                    }
                    plot_ui.points(Points::new(PlotPoints::new(arrival_points)).radius(4.0).color(egui::Color32::GREEN).name("On Time"));
                    plot_ui.points(Points::new(PlotPoints::new(violation_points)).radius(6.0).color(egui::Color32::RED).name("Violation"));

                    // Wait times
                    let mut wait_bars = Vec::new();
                    for (_i, visit) in schedule.iter().enumerate() {
                        if visit.wait_time > 0.0 {
                            let mut bar = Bar::new(visit.node_idx as f64, visit.wait_time as f64)
                                .base_offset(visit.arrival_time as f64)
                                .fill(egui::Color32::YELLOW)
                                .name("Wait");
                            bar.orientation = Orientation::Horizontal;
                            wait_bars.push(bar);
                        }
                    }
                    plot_ui.bar_chart(BarChart::new(wait_bars).name("Wait Time").color(egui::Color32::YELLOW));
                }
            });
    });
}

