use eframe::egui;

use crate::factories::AlgoType;
use crate::gui::state::{AppState, AppPhase};
use crate::neighborhood::NeighborhoodType;
use crate::eval::EvaluationType;

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
                      ui.label(format!("Loaded: {} nodes", inst.size()));
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
                      ui.radio_value(&mut state.algo_type, AlgoType::GeneticAlgorithm, "Genetic Algorithm");
                      ui.radio_value(&mut state.algo_type, AlgoType::HillClimbing, "Hill Climbing");
                      ui.radio_value(&mut state.algo_type, AlgoType::AntColonyOptimization, "Ant Colony Optimization");
                  });
                  
                  ui.horizontal(|ui| {
                      ui.label("Neighborhood:");
                      ui.radio_value(&mut state.algo_config.neighborhood, NeighborhoodType::Swap, "Swap");
                      ui.radio_value(&mut state.algo_config.neighborhood, NeighborhoodType::TwoOpt, "2-Opt");
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
                  
                  // Simulated Annealing parameters
                  if state.algo_type == AlgoType::SimulatedAnnealing {
                      ui.label(egui::RichText::new("Simulated Annealing").strong());
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_temp, 1.0..=100000.0).text("Initial Temperature"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_cooling, 0.9..=1.0).text("Cooling Rate"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_stopping, 0.0001..=10.0).text("Stopping Temperature"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_acceptance_smoothing, 0.0..=1.0).text("Acceptance Smoothing"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_initial_acceptance_rate, 0.0..=1.0).text("Initial Acceptance Rate"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_delta_fitness_smoothing, 0.0..=1.0).text("Delta Fitness Smoothing"));
                      ui.add(egui::Slider::new(&mut state.algo_config.sa_backtracking_interval, 0..=100000).text("Backtracking Interval (0 = off)"));
                  }

                  // Genetic Algorithm parameters
                  if state.algo_type == AlgoType::GeneticAlgorithm {
                      ui.label(egui::RichText::new("Genetic Algorithm").strong());
                      ui.add(egui::Slider::new(&mut state.algo_config.ga_crossover_rate, 0.0..=1.0).text("Crossover Rate"));
                      ui.horizontal(|ui| {
                          ui.label("Crossover Type:");
                          ui.radio_value(&mut state.algo_config.ga_crossover_type, crate::algorithms::CrossoverType::PMX, "PMX");
                          ui.radio_value(&mut state.algo_config.ga_crossover_type, crate::algorithms::CrossoverType::OX, "OX");
                      });
                      ui.add(egui::Slider::new(&mut state.algo_config.ga_elitism_rate, 0.0..=1.0).text("Elitism Rate"));
                      ui.add(egui::Slider::new(&mut state.algo_config.ga_competition_participation_rate, 0.0..=1.0).text("Competition Participation"));
                      ui.horizontal(|ui| {
                          ui.label("Competition Type:");
                          ui.radio_value(&mut state.algo_config.ga_competition_type, crate::algorithms::CompetitionType::Tournament, "Tournament");
                          ui.radio_value(&mut state.algo_config.ga_competition_type, crate::algorithms::CompetitionType::Roulette, "Roulette");
                      });
                      ui.add(egui::Slider::new(&mut state.algo_config.population_size, 10..=500).text("Population Size"));
                  }

                  // Hill Climbing parameters
                  if state.algo_type == AlgoType::HillClimbing {
                      ui.label(egui::RichText::new("Hill Climbing").strong());
                      ui.add(egui::Slider::new(&mut state.algo_config.hc_step, 1..=100).text("Step Size"));
                  }

                  // Ant Colony Optimization parameters
                  if state.algo_type == AlgoType::AntColonyOptimization {
                      ui.label(egui::RichText::new("Ant Colony Optimization").strong());
                      ui.add(egui::Slider::new(&mut state.algo_config.aco_evaporation, 0.0..=1.0).text("Evaporation Rate"));
                      ui.add(egui::Slider::new(&mut state.algo_config.aco_alpha, 0.0..=10.0).text("Alpha (Pheromone)"));
                      ui.add(egui::Slider::new(&mut state.algo_config.aco_beta, 0.0..=10.0).text("Beta (Heuristic)"));
                      ui.add(egui::Slider::new(&mut state.algo_config.aco_deposit, 0.1..=100.0).text("Pheromone Deposit"));
                      ui.add(egui::Slider::new(&mut state.algo_config.population_size, 10..=500).text("Population Size"));
                  }
                  
                  ui.separator();
                  
                  // Evaluation parameters
                  if state.evaluation_type == EvaluationType::Weighted {
                      ui.label(egui::RichText::new("Weighted Evaluation").strong());
                      ui.add(egui::Slider::new(&mut state.eval_config.total_distance_weight, 0.0..=10.0).text("Distance Weight"));
                      ui.add(egui::Slider::new(&mut state.eval_config.violation_time_weight, 0.0..=10000.0).text("Violation Time Weight"));
                      ui.add(egui::Slider::new(&mut state.eval_config.total_time_weight, 0.0..=10.0).text("Total Time Weight"));
                      ui.add(egui::Slider::new(&mut state.eval_config.delay_weight, 0.0..=10.0).text("Delay Weight"));
                  }

                  if state.evaluation_type == EvaluationType::Lexicographic {
                      ui.label(egui::RichText::new("Lexicographic Evaluation").strong());
                      ui.checkbox(&mut state.eval_config.lexicographic_distance_first, "Prioritize Distance over Violation");
                  }
              });
  
              ui.add_space(10.0);
  
              ui.group(|ui| {
                  ui.heading("Execution");
                  ui.add(egui::Slider::new(&mut state.steps_per_frame, 1..=10000).text("Steps/Frame"));
                  ui.add(egui::Slider::new(&mut state.algo_config.max_steps, 100..=100000000).text("Max Steps"));
                  ui.add(egui::Slider::new(&mut state.parallel_runs_count, 1..=1000).text("Parallel Runs"));
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
