use eframe::egui;
use egui_plot::{Arrows, Line, Plot, PlotPoint, PlotPoints, Points, Text};
use crate::gui::state::RunState;
use crate::problem::Instance;

pub fn show(ui: &mut egui::Ui, run_state: &RunState, instance: &Option<Instance>) {
    ui.push_id("route_plot", |ui| {
        Plot::new("TSP Route")
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                if let Some(instance) = instance {
                    // Draw cities
                    let points: PlotPoints = instance.graph.iter().map(|n| [n.x as f64, n.y as f64]).collect();
                    plot_ui.points(Points::new(points).radius(8.0).color(egui::Color32::RED));
                    
                    // Draw node IDs
                    for (i, n) in instance.graph.iter().enumerate() {
                        plot_ui.text(Text::new(
                            PlotPoint::new(n.x as f64, n.y as f64 + 1.0 ), 
                            egui::RichText::new(i.to_string()).size(16.0).strong()
                        ).color(egui::Color32::WHITE));
                    }

                    // Draw path
                    if !run_state.current_solution_path.is_empty() {
                        let mut line_points: Vec<[f64; 2]> = Vec::new();
                        let mut arrow_origins = Vec::new();
                        let mut arrow_tips = Vec::new();

                        for i in 0..run_state.current_solution_path.len() {
                            let from_idx = run_state.current_solution_path[i] as usize;
                            let next_i = (i + 1) % run_state.current_solution_path.len();
                            let to_idx = run_state.current_solution_path[next_i] as usize;
                            
                            let n1 = &instance.graph[from_idx];
                            let n2 = &instance.graph[to_idx];
                            
                            line_points.push([n1.x as f64, n1.y as f64]);

                            // Calculate arrow (fixed size, centered)
                            let dx = n2.x as f64 - n1.x as f64;
                            let dy = n2.y as f64 - n1.y as f64;
                            let len = (dx * dx + dy * dy).sqrt();
                            
                            if len > 0.1 {
                                let dir_x = dx / len;
                                let dir_y = dy / len;
                                
                                // Fixed arrow length, but not larger than 60% of segment
                                let arrow_len = 3.0f64.min(len * 0.6);
                                
                                let mid_x = (n1.x as f64 + n2.x as f64) / 2.0;
                                let mid_y = (n1.y as f64 + n2.y as f64) / 2.0;
                                
                                // Center the arrow
                                let origin_x = mid_x - (dir_x * arrow_len * 0.5);
                                let origin_y = mid_y - (dir_y * arrow_len * 0.5);
                                
                                // Calculate tip position (absolute coordinates)
                                let tip_x = origin_x + (dir_x * arrow_len);
                                let tip_y = origin_y + (dir_y * arrow_len);
                                
                                arrow_origins.push([origin_x, origin_y]);
                                arrow_tips.push([tip_x, tip_y]);
                            }
                        }
                        
                        // Close loop for line
                        if let Some(first) = run_state.current_solution_path.first() {
                            let n = &instance.graph[*first as usize];
                            line_points.push([n.x as f64, n.y as f64]);
                        }
                        
                        plot_ui.line(Line::new(PlotPoints::new(line_points)).color(egui::Color32::BLUE));
                        plot_ui.arrows(Arrows::new(arrow_origins, arrow_tips).color(egui::Color32::from_rgb(255, 165, 0))); // Orange arrows
                    }
                }
            });
    });
    
    ui.separator();
    ui.label(egui::RichText::new("Path Sequence:").strong());
    egui::ScrollArea::vertical().max_height(60.0).show(ui, |ui| {
        ui.label(run_state.current_solution_path.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" -> "));
    });
}
