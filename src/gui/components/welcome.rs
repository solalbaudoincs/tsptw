use eframe::egui;
use crate::gui::state::{AppState, AlgoType, NeighborhoodType, EvaluationType, AppPhase};

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
        //ui.set_max_width(600.0);
        ui.vertical_centered(|ui| {
            ui.set_max_width(600.0);
          ui.heading("Configuration");
          egui::ScrollArea::vertical().show(ui, |ui| {
              //the heading takes all the vertical space 
     
              //ui.add_space(20.0)
              ui.group(|ui| {
                  ui.heading("Instance");
                  ui.horizontal(|ui| {
                      ui.label("Path:");
                      ui.text_edit_singleline(&mut state.instance_path);
                      if ui.button("Load").clicked() {
                          state.load_instance();
                      }
                  });
                  if let Some(inst) = &state.instance {
                      ui.label(format!("Loaded: {} nodes", inst.graph.len()));
                  } else {
                      ui.colored_label(egui::Color32::RED, "No instance loaded");
                  }
              });
              
              ui.add_space(10.0);
  
              ui.group(|ui| {
                  ui.heading("Algorithm Selection");
                  ui.horizontal(|ui| {
                      ui.label("Algorithm:");
                      ui.radio_value(&mut state.algo_type, AlgoType::SimulatedAnnealing, "Simulated Annealing");
                  });
                  
                  ui.horizontal(|ui| {
                      ui.label("Neighborhood:");
                      ui.radio_value(&mut state.neighborhood_type, NeighborhoodType::Swap, "Swap");
                      ui.radio_value(&mut state.neighborhood_type, NeighborhoodType::TwoOpt, "2-Opt");
                  });
  
                  ui.horizontal(|ui| {
                      ui.label("Evaluation:");
                      ui.radio_value(&mut state.evaluation_type, EvaluationType::Weighted, "Weighted");
                      ui.radio_value(&mut state.evaluation_type, EvaluationType::Lexicographic, "Lexicographic");
                  });
              });
  
              ui.add_space(10.0);
  
              ui.group(|ui| {
                  ui.heading("Parameters");
                  if state.algo_type == AlgoType::SimulatedAnnealing {
                      ui.add(egui::Slider::new(&mut state.sa_temp, 1.0..=100000.0).text("Initial Temp"));
                      ui.add(egui::Slider::new(&mut state.sa_cooling, 0.9..=1.0).text("Cooling Rate"));
                  }
                  
                  if state.evaluation_type == EvaluationType::Weighted {
                      ui.add(egui::Slider::new(&mut state.violation_coefficient, 0.0..=10000.0).text("Violation Coeff"));
                  }
              });
  
              ui.add_space(10.0);
  
              ui.group(|ui| {
                  ui.heading("Execution");
                  ui.add(egui::Slider::new(&mut state.steps_per_frame, 1..=10000).text("Steps/Frame"));
                  ui.add(egui::Slider::new(&mut state.max_steps, 100..=1000000).text("Max Steps"));
                  ui.add(egui::Slider::new(&mut state.parallel_runs_count, 1..=200).text("Parallel Runs"));
              });
  
              ui.add_space(20.0);
  
              let start_enabled = state.instance.is_some();
              ui.add_enabled_ui(start_enabled, |ui| {
                  let btn = egui::Button::new("START SIMULATION").min_size(egui::Vec2::new(200.0, 50.0));
                  if ui.add(btn).clicked() {
                      state.phase = AppPhase::Running;
                      for _ in 0..state.parallel_runs_count {
                          state.start_new_run();
                      }
                  }
              });
              if !start_enabled {
                  ui.label("Please load an instance first.");
              }
          });  
        });
        
}
