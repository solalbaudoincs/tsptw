use eframe::egui;
use crate::gui::state::AppState;
use crate::gui::tabs;

pub fn show(ui: &mut egui::Ui, state: &mut AppState, run_idx: usize) {
    // Detailed View (Dashboard)
    if ui.button("Back to Grid").clicked() {
        state.selected_run_index = None;
    }
    
    if run_idx < state.runs.len() {
        // We need to split state borrowing here to avoid conflicts if possible, 
        // but since we are passing mutable state to tabs, we might need to be careful.
        // However, in the original code, it was all `self.state`.
        // Let's try to replicate the borrowing pattern.
        
        let run = &mut state.runs[run_idx];
        let instance = &state.instance;
        let log_scale = &mut state.violation_log_scale;
        let left_col_ratio = &mut state.left_col_ratio;
        let right_top_ratio = &mut state.right_top_ratio;

        use egui::{Layout, Align, Vec2};
        
        // Dashboard with draggable splitter
        let available = ui.available_size();
        let divider_w = 6.0f32;

        // compute widths from ratio
        let left_w = (available.x - divider_w) * left_col_ratio.clamp(0.1, 0.9);
        let right_w = available.x - divider_w - left_w;

        ui.horizontal(|ui| {
            // Left column (Route)
            ui.allocate_ui_with_layout(Vec2::new(left_w, available.y), Layout::top_down(Align::Min), |ui| {
                tabs::route::show(ui, run, instance);
            });

            // Vertical divider (draggable)
            let (divider_rect, _divider_resp) = ui.allocate_exact_size(Vec2::new(divider_w, available.y), egui::Sense::drag());
            ui.painter().rect_filled(divider_rect, 0.0, egui::Color32::DARK_GRAY);
            let id = ui.id().with("left_divider");
            let resp = ui.interact(divider_rect, id, egui::Sense::drag());
            if resp.dragged() {
                let delta = resp.drag_delta().x;
                let ratio_delta = delta / available.x;
                *left_col_ratio = (*left_col_ratio + ratio_delta).clamp(0.2, 0.8);
            }

            // Right column (Metrics above, Gantt below)
            ui.allocate_ui_with_layout(Vec2::new(right_w, available.y), Layout::top_down(Align::Min), |right_ui| {
                // vertical split inside right column
                let right_av = right_ui.available_size();
                let h_div_h = 6.0f32;
                let top_h = (right_av.y - h_div_h) * right_top_ratio.clamp(0.1, 0.9);
                let bottom_h = right_av.y - h_div_h - top_h;

                // Top (Metrics)
                right_ui.allocate_ui_with_layout(Vec2::new(right_av.x, top_h), Layout::top_down(Align::Min), |ui| {
                    tabs::metrics::show(ui, run, log_scale);
                });

                // Horizontal divider (draggable)
                let (hdiv_rect, _hdiv_resp) = right_ui.allocate_exact_size(Vec2::new(right_av.x, h_div_h), egui::Sense::drag());
                right_ui.painter().rect_filled(hdiv_rect, 0.0, egui::Color32::DARK_GRAY);
                let hid = right_ui.id().with("right_divider");
                let hresp = right_ui.interact(hdiv_rect, hid, egui::Sense::drag());
                if hresp.dragged() {
                    let delta = hresp.drag_delta().y;
                    let ratio_delta = delta / right_av.y;
                    *right_top_ratio = (*right_top_ratio + ratio_delta).clamp(0.1, 0.9);
                }

                // Bottom (Gantt)
                right_ui.allocate_ui_with_layout(Vec2::new(right_av.x, bottom_h), Layout::top_down(Align::Min), |ui| {
                    tabs::gantt::show(ui, run, instance);
                });
            });
        });
    }
}
