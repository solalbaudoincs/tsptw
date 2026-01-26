# tsptw

A Rust solver for the Traveling Salesman Problem with Time Windows (TSPTW), implemented from scratch.

## Features

- **Metaheuristics**: Genetic Algorithm, Simulated Annealing, Ant Colony Optimization, Variable Neighborhood Search, Hill Climbing
- **Neighborhood operators**: 2-opt, swap
- **Hyperparameter tuning**: Grid search, Bayesian optimization
- **Interactive GUI**: For real-time visualization of routes, time windows, and solver convergence.
- **Parallel Computing**: Optimized for performance using `rayon` for concurrent evaluations.
- **Modular Architecture**: Extensible factory pattern for algorithms, evaluation metrics, and neighborhood operators.

## Usage

```bash
# Run the GUI
cargo run --release -- --gui

# CLI options
cargo run --release -- --help
```

## Structure

```
src/
├── algorithms/    # GA, SA, ACO, VNS, Hill Climbing
├── neighborhood/  # 2-opt, swap
├── hpo/           # Hyperparameter optimization
├── eval/          # Solution evaluation
├── gui/           # Graphical interface
└── io/            # Instance parsing
```
