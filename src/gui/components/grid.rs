use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use crate::gui::state::AppState;
use super::statistics::draw_aggregated_plot;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    use egui::{Vec2, Layout, Align};
    
    let available = ui.available_size();
    let aggregation_height = 250.0;
    let grid_height = (available.y - aggregation_height).max(100.0);

    // Grid Area
    ui.allocate_ui_with_layout(Vec2::new(available.x, grid_height), Layout::top_down(Align::Min), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let card_width = 300.0;
            let card_height = 220.0;
            let spacing = 10.0;
            let available_width = ui.available_width();
            let cols = ((available_width + spacing) / (card_width + spacing)).floor() as usize;
            let cols = cols.max(1);

            egui::Grid::new("runs_grid")
                .spacing(Vec2::new(spacing, spacing))
                .show(ui, |ui| {
                    for (i, run) in state.runs.iter().enumerate() {
                        ui.push_id(i, |ui| {
                            let (rect, resp) = ui.allocate_exact_size(Vec2::new(card_width, card_height), egui::Sense::click());
                            
                            ui.painter().rect_filled(rect, 5.0, egui::Color32::from_gray(50));
                            if resp.hovered() {
                                ui.painter().rect_stroke(rect, 5.0, egui::Stroke::new(2.0, egui::Color32::WHITE), egui::StrokeKind::Inside);
                            }
                            
                            if resp.clicked() {
                                state.selected_run_index = Some(i);
                            }
                            
                            // Draw content inside card
                            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect.shrink(10.0)).layout(egui::Layout::top_down(egui::Align::Min)), |ui| {
                                ui.label(egui::RichText::new(&run.name).strong().size(16.0));
                                ui.label(format!("Iter: {}", run.history.last().map(|l| l.iteration).unwrap_or(0)));
                                if let Some(last) = run.history.last() {
                                    ui.label(format!("Dist: {:.2}", last.current_dist));
                                    ui.label(format!("Viol: {:.2}", last.current_viol));
                                }
                                if run.is_running {
                                    ui.colored_label(egui::Color32::GREEN, "Running");
                                } else {
                                    ui.colored_label(egui::Color32::RED, "Stopped");
                                }

                                // Sparklines
                                let history_len = run.history.len();
                                if history_len > 1 {
                                    let points: Vec<[f64; 2]> = run.history.iter().enumerate()
                                        .map(|(i, h)| [i as f64, h.current_dist as f64])
                                        .collect();
                                    
                                    ui.add(egui::Label::new("Distance"));
                                    Plot::new(format!("spark_dist_{}", i))
                                        .show_axes([false, false])
                                        .show_grid([false, false])
                                        .allow_drag(false)
                                        .allow_zoom(false)
                                        .allow_scroll(false)
                                        .height(40.0)
                                        .show(ui, |plot_ui| {
                                            plot_ui.line(Line::new(PlotPoints::new(points)).color(egui::Color32::LIGHT_BLUE));
                                        });

                                    let points_viol: Vec<[f64; 2]> = run.history.iter().enumerate()
                                        .map(|(i, h)| [i as f64, h.current_viol as f64])
                                        .collect();
                                    
                                    ui.add(egui::Label::new("Violation"));
                                    Plot::new(format!("spark_viol_{}", i))
                                        .show_axes([false, false])
                                        .show_grid([false, false])
                                        .allow_drag(false)
                                        .allow_zoom(false)
                                        .allow_scroll(false)
                                        .height(40.0)
                                        .show(ui, |plot_ui| {
                                            plot_ui.line(Line::new(PlotPoints::new(points_viol)).color(egui::Color32::LIGHT_RED));
                                        });
                                }
                            });
                        });

                        if (i + 1) % cols == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    });

    ui.separator();

    // Aggregation Area (Bottom)
    ui.allocate_ui_with_layout(Vec2::new(available.x, aggregation_height), Layout::top_down(Align::Min), |ui| {
        ui.heading("Aggregated Results");
        ui.horizontal(|ui| {
            let plot_width = (ui.available_width() / 2.0) - 10.0;
            
            ui.vertical(|ui| {
                ui.label("Distance (Mean ± 95% CI)");
                Plot::new("Aggregated_Dist_Grid")
                    .width(plot_width)
                    .height(aggregation_height - 30.0)
                    .show(ui, |plot_ui| {
                        draw_aggregated_plot(state, plot_ui, |h| h.current_dist, egui::Color32::BLUE);
                    });
            });

            ui.vertical(|ui| {
                ui.label("Violation (Mean ± 95% CI)");
                Plot::new("Aggregated_Viol_Grid")
                    .width(plot_width)
                    .height(aggregation_height - 30.0)
                    .show(ui, |plot_ui| {
                        draw_aggregated_plot(state, plot_ui, |h| h.current_viol, egui::Color32::RED);
                    });
            });
        });
    });
}
