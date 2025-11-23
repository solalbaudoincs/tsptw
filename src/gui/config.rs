use eframe::egui;
use crate::gui::state::{AppState, AlgoType};

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Configuration");
    ui.text_edit_singleline(&mut state.instance_path);
    if ui.button("Load Instance").clicked() {
        state.load_instance();
    }
    
    ui.separator();
    ui.label("Algorithm");
    ui.radio_value(&mut state.algo_type, AlgoType::SimulatedAnnealing, "Simulated Annealing");
    
    if state.algo_type == AlgoType::SimulatedAnnealing {
        ui.add(egui::Slider::new(&mut state.sa_temp, 1.0..=10000.0).text("Initial Temp"));
        ui.add(egui::Slider::new(&mut state.sa_cooling, 0.9..=0.99999).text("Cooling Rate"));
    }
    
    ui.separator();
    ui.label("Evaluation");
    ui.add(egui::Slider::new(&mut state.violation_coefficient, 0.0..=10000.0).text("Violation Coeff"));

    ui.separator();
    ui.add(egui::Slider::new(&mut state.steps_per_frame, 1..=1000).text("Steps/Frame"));
    ui.add(egui::Slider::new(&mut state.max_steps, 100..=1000000).text("Max Steps"));
    
    ui.separator();
    ui.add(egui::Slider::new(&mut state.parallel_runs_count, 1..=50).text("Parallel Runs"));

    if ui.button("Start New Run(s)").clicked() {
        for _ in 0..state.parallel_runs_count {
            state.start_new_run();
        }
    }
    
    if ui.button("Clear All Runs").clicked() {
        state.runs.clear();
        state.selected_run_index = None;
    }
}

