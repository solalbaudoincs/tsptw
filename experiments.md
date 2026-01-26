## Comprehensive Experiment Specification for Simulated Annealing Performance Analysis

This specification provides a complete experimental protocol to systematically evaluate your **Simulated Annealing (SA)** implementation for the Traveling Salesman Problem with Time Windows (TSPTW) on the `experiments` branch of [tsptw](https://github.com/solalbaudoincs/tsptw/tree/experiments). The study investigates neighborhood operators, evaluation weight configurations, and initialization strategies across all test instances.

***

## Executive Summary

**Total Experimental Runs**: 4,800 SA executions  
**Test Instances**: `inst1`, `inst2`, `inst3`, `inst_concours`  
**Parallel Runs per Configuration**: 200  
**Primary Research Questions**:
1. How does neighborhood operator choice (Swap vs. 2-opt vs. Alternating vs. Bandit) affect solution quality and convergence speed?
2. What is the sensitivity of SA performance to weighted evaluation parameters?
3. Does warmup initialization provide statistically significant improvements over cold starts with calibrated temperatures?
4. What are the optimal hyperparameter combinations for each instance's structural characteristics?

***

## Phase 0: Temperature Calibration Protocol

### Objective
Establish fair baseline temperatures for cold-start experiments by measuring the natural equilibrium temperature reached during warmup phases. This eliminates initialization bias when comparing warmup vs. no-warmup configurations.

### Experimental Setup

**Algorithm Configuration**:
```json
{
  "Algorithm": "Simulated Annealing",
  "Neighborhood": ["Swap", "2-opt"],
  "Evaluation": "Weighted",
  "Parameters": {
    "Initial Temperature": 100000,
    "Cooling Rate": 0.99999,
    "Warmup Steps": 10000,
    "Stopping Temperature": 0.0001,
    "Acceptance Smoothing": 1.0,
    "Initial Acceptance Rate": 0.999,
    "Delta Fitness Smoothing": 1.0
  },
  "Weighted Evaluation": {
    "Distance Weight": 0.0,
    "Violation Time Weight": 4.0,
    "Total Time Weight": 4.0,
    "Delay Weight": 2.0
  }
}
```

**Data Collection**:
- **Sampling Points**: Record temperature at steps 1000, 2000, ..., 10000 during warmup
- **Final Temperature**: Capture \( T_{\text{final}} \) at step 10,000 for all 200 runs
- **Distribution Metrics**: Compute mean \( \mu(T) \), standard deviation \( \sigma(T) \), median, IQR, min/max

### Output Files

| File | Format | Content | Purpose |
|------|--------|---------|---------|
| `{inst}_warmup_temp_trajectory.csv` | CSV | `run_id, step, temperature` | Convergence analysis |
| `{inst}_warmup_temp_final.csv` | CSV | `run_id, final_temp, final_fitness` | Distribution plotting |
| `{inst}_warmup_stats.json` | JSON | `{mean, std, median, q1, q3}` | LaTeX table generation |

### Analysis Deliverables

1. **Temperature Distribution Histogram**: Kernel density estimate (KDE) overlay showing \( T_{\text{final}} \) distribution per instance
2. **Convergence Profile**: Mean temperature trajectory with Q1-Q3 ribbons across all runs
3. **Statistical Summary Table**:
```latex
\begin{tabular}{lcccc}
\toprule
Instance & $\mu(T)$ & $\sigma(T)$ & Median & IQR \\
\midrule
inst1 & 2547.3 & 312.5 & 2501.2 & [2301.1, 2799.4] \\
inst2 & ... & ... & ... & ... \\
\bottomrule
\end{tabular}
```

**Validation Criterion**: If \( \sigma(T) / \mu(T) > 0.3 \), flag high variance and consider extended warmup duration.

***

## Phase 1: Neighborhood Operator Comparison (EXP-01)

### Objective
Quantify the performance differences between **Swap** and **2-opt** neighborhoods under both warmup-enabled and calibrated cold-start conditions.

### Experimental Matrix

| Config ID | Neighborhood | Warmup | Initial Temp | Rationale |
|-----------|--------------|--------|--------------|-----------|
| **EXP-01A** | Swap | Yes | 100000 (natural) | Baseline with warmup |
| **EXP-01B** | Swap | No | \( \mu(T_{\text{warmup}}) \) | Fair cold-start |
| **EXP-01C** | 2-opt | Yes | 100000 (natural) | Operator comparison |
| **EXP-01D** | 2-opt | No | \( \mu(T_{\text{warmup}}) \) | Fair cold-start |

**Total Runs**: 4 configs × 4 instances × 200 runs = **3,200 executions**

### Fixed Parameters
```json
{
  "Cooling Rate": 0.99999,
  "Stopping Temperature": 0.0001,
  "Acceptance Smoothing": 1.0,
  "Initial Acceptance Rate": 0.999,
  "Delta Fitness Smoothing": 1.0,
  "Steps/Frame": 10000,
  "Max Steps": 100000,
  "Weights": {
    "Distance Weight": 0.0,
    "Violation Time Weight": 4.0,
    "Total Time Weight": 4.0,
    "Delay Weight": 2.0
  }
}
```

### Metrics to Collect

**Per-Run Metrics** (`{inst}_{config}_runs.csv`):
```csv
run_id,config,best_fitness,best_step,final_fitness,violations,feasible,total_distance,total_time,execution_time_ms
0,EXP-01A,8524.3,45200,8524.3,0,1,2314.5,6209.8,12453
1,EXP-01A,8901.2,67800,8901.2,2,0,2456.1,6445.1,12389
...
```

**Per-Step Aggregated Metrics** (`{inst}_{config}_convergence.csv`):
```csv
step,mean_fitness,median_fitness,q1_fitness,q3_fitness,p10_fitness,p90_fitness,feasible_rate
0,15234.5,14987.2,13456.1,16789.3,12345.6,18234.5,0.02
500,13456.7,13201.4,11987.6,14567.8,10456.3,16234.1,0.15
1000,11987.3,11654.2,10345.7,13123.4,9234.5,14567.8,0.34
...
```

### Statistical Analysis

1. **Wilcoxon Signed-Rank Test**: Compare Swap vs. 2-opt best fitness distributions (paired by run seed)
2. **Convergence Speed**: Time to reach 90% of best-ever fitness
3. **Robustness**: Coefficient of variation \( \text{CV} = \sigma / \mu \) of best fitness across runs

### Visualization Outputs

**Convergence Plots** (`{inst}_exp01_convergence.pgf`):
```latex
\begin{tikzpicture}
\begin{axis}[
    xlabel={Step},
    ylabel={Fitness},
    legend pos=north east,
    width=0.9\textwidth,
    height=6cm,
    grid=major
]
% Swap with warmup (EXP-01A)
\addplot[name path=swap_upper, draw=none] table[x=step,y=q3_fitness] {inst1_exp01a.csv};
\addplot[name path=swap_lower, draw=none] table[x=step,y=q1_fitness] {inst1_exp01a.csv};
\addplot[fill=blue!20] fill between[of=swap_upper and swap_lower];
\addplot[blue, thick] table[x=step,y=median_fitness] {inst1_exp01a.csv};
\addlegendentry{Swap + Warmup (IQR)}

% 2-opt with warmup (EXP-01C)
\addplot[name path=twoopt_upper, draw=none] table[x=step,y=q3_fitness] {inst1_exp01c.csv};
\addplot[name path=twoopt_lower, draw=none] table[x=step,y=q1_fitness] {inst1_exp01c.csv};
\addplot[fill=red!20] fill between[of=twoopt_upper and twoopt_lower];
\addplot[red, thick] table[x=step,y=median_fitness] {inst1_exp01c.csv};
\addlegendentry{2-opt + Warmup (IQR)}
\end{axis}
\end{tikzpicture}
```

**Box Plot Comparison** (`{inst}_exp01_boxplot.pgf`):
- X-axis: Configuration ID
- Y-axis: Best fitness achieved
- Overlay: Statistical significance indicators (*, **, ***)

***

## Phase 2: Advanced Neighborhood Strategies (EXP-02)

### Objective
Evaluate adaptive and hybrid neighborhood selection mechanisms against static operators.

### Experimental Configurations

| Config ID | Neighborhood | Description | Hypothesis |
|-----------|--------------|-------------|------------|
| **EXP-02A** | Alternating | Toggle Swap ↔ 2-opt every iteration | Diversification via forced switching |
| **EXP-02B** | Bandit | UCB1 multi-armed bandit selection | Exploitation of best-performing operator |

**Initialization**: Both use \( \mu(T_{\text{warmup}}) \) from Phase 0, **no warmup**  
**Baseline Comparison**: EXP-01B (Swap, cold-start) and EXP-01D (2-opt, cold-start)  
**Total Runs**: 2 configs × 4 instances × 200 runs = **1,600 executions**

### Bandit Algorithm Details

**UCB1 Formula**:
\[
\text{UCB}_i = \bar{X}_i + C \sqrt{\frac{\ln N}{n_i}}
\]
where:
- \( \bar{X}_i \): Average improvement from operator \( i \) (Swap or 2-opt)
- \( N \): Total operator selections so far
- \( n_i \): Number of times operator \( i \) was selected
- \( C \): Exploration constant (default: \( C = \sqrt{2} \))

**Reward Signal**: Negative delta fitness (improvement) after applying operator

### Additional Metrics

**Bandit-Specific** (`{inst}_exp02b_bandit_stats.csv`):
```csv
run_id,step,swap_selections,twoopt_selections,swap_avg_reward,twoopt_avg_reward
0,1000,547,453,-12.4,-8.7
0,2000,1123,877,-10.2,-9.1
...
```

### Visualization Outputs

1. **Operator Selection Frequency**: Stacked area chart showing \( \frac{n_{\text{swap}}}{N} \) vs. \( \frac{n_{\text{2opt}}}{N} \) over time
2. **Reward Trajectory**: Mean reward per operator with confidence intervals
3. **Comparative Convergence**: Overlay EXP-01B/D, EXP-02A, EXP-02B on same plot

***

## Phase 3: Weight Sensitivity Analysis (EXP-03)

### Objective
Determine how weighted evaluation parameters affect feasibility, objective quality, and convergence behavior.

### Weight Configurations

| Config ID | Distance | Violation | Time | Delay | Interpretation |
|-----------|----------|-----------|------|-------|----------------|
| **EXP-03A** (Baseline) | 0.0 | 4.0 | 4.0 | 2.0 | Balanced constraint/objective |
| **EXP-03B** (Hard) | 0.0 | 10.0 | 2.0 | 1.0 | Prioritize feasibility |
| **EXP-03C** (Relaxed) | 0.0 | 2.0 | 4.0 | 4.0 | Emphasize time quality |
| **EXP-03D** (Distance) | 1.0 | 5.0 | 3.0 | 2.0 | Include distance minimization |

**Fixed**: Bandit neighborhood, warmup enabled (natural init temp 100000)  
**Total Runs**: 4 configs × 4 instances × 200 runs = **3,200 executions**

### Instance-Specific Hypotheses

- **inst1**: Lower violation weight sufficient (few time windows)
- **inst2**: Moderate weights balance feasibility and quality
- **inst3**: High delay weight critical (tight time windows)
- **inst_concours**: Hard configuration necessary for feasible solutions

### Feasibility Analysis

**Feasibility Rate**: \( F_r = \frac{\#\{\text{runs with 0 violations}\}}{200} \)

**LaTeX Table**:
```latex
\begin{tabular}{lcccc}
\toprule
Config & inst1 $F_r$ & inst2 $F_r$ & inst3 $F_r$ & concours $F_r$ \\
\midrule
Baseline & 0.98 & 0.92 & 0.78 & 0.45 \\
Hard & 1.00 & 0.99 & 0.95 & 0.87 \\
Relaxed & 0.95 & 0.84 & 0.61 & 0.23 \\
\bottomrule
\end{tabular}
```

### Pareto Frontier Analysis

For each instance, plot:
- X-axis: Total distance (km)
- Y-axis: Total time window violations (seconds)
- Color: Weight configuration
- Shape: Feasible (circle) vs. Infeasible (cross)

**Objective**: Identify weight settings that achieve feasibility with minimal objective degradation.

***

## Phase 4: Warmup Ablation Study (EXP-04)

### Objective
Isolate the marginal benefit of warmup initialization by controlling for temperature using Phase 0 calibration.

### Experimental Design

| Config ID | Neighborhood | Warmup | Initial Temp | Notes |
|-----------|--------------|--------|--------------|-------|
| **EXP-04A** | Bandit | Yes | 100000 (natural) | Full warmup benefit |
| **EXP-04B** | Bandit | No | \( \mu(T_{\text{warmup}}) \) | Calibrated cold-start |

**Fixed**: Baseline weights (EXP-03A)  
**Total Runs**: 2 configs × 4 instances × 200 runs = **1,600 executions**

### Value-Added Metrics

**Warmup Efficiency**:
\[
\eta_{\text{warmup}} = \frac{F_{\text{best}}^{\text{no-warmup}} - F_{\text{best}}^{\text{warmup}}}{T_{\text{warmup}}}
\]
where \( T_{\text{warmup}} \) is the execution time of warmup phase (ms).

**Initial Solution Quality**:
Compare fitness at step 0 (post-warmup) vs. step 0 (random initialization).

### Statistical Test

**Paired t-test**: \( H_0 \): Warmup does not improve best fitness  
**Effect Size**: Cohen's d for practical significance

***

## Consolidated Reporting Framework

### Master Results Table

**Summary Statistics** (`master_results.tex`):
```latex
\begin{tabular}{llcccccc}
\toprule
Phase & Config & Instance & Best & Mean & Std & $F_r$ & Time (s) \\
\midrule
0 & Calib-Swap & inst1 & - & - & - & - & 124.5 \\
1 & EXP-01A & inst1 & 8524.3 & 8912.7 & 287.4 & 0.98 & 132.1 \\
1 & EXP-01B & inst1 & 8601.2 & 9034.5 & 312.8 & 0.96 & 128.7 \\
... & ... & ... & ... & ... & ... & ... & ... \\
\bottomrule
\end{tabular}
```

### Visualization Asset Checklist

| Asset Type | Filename Pattern | LaTeX Integration | Purpose |
|------------|------------------|-------------------|---------|
| Convergence plot | `{inst}_{exp}_conv.pgf` | `\input{}` | Fill-between ribbons |
| Box plot | `{exp}_boxplot.pgf` | `\input{}` | Cross-instance comparison |
| Histogram | `{inst}_temp_hist.pgf` | `\input{}` | Phase 0 calibration |
| Pareto front | `{inst}_pareto.pgf` | `\input{}` | Weight sensitivity |
| Bandit dynamics | `{inst}_bandit.pgf` | `\input{}` | Operator selection |
| Best tour | `{inst}_best_tour.pdf` | `\includegraphics{}` | Visual validation |

### Data Export Standards

**CSV Column Headers**:
```
step,run_id,fitness,temperature,feasible,violations,distance,time,delay,operator_used
```

**JSON Metadata** (`{exp}_metadata.json`):
```json
{
  "experiment_id": "EXP-01A",
  "instance": "inst1",
  "config": {...},
  "runs": 200,
  "seed_range": [0, 199],
  "execution_date": "2026-01-26",
  "total_runtime_seconds": 2647.3
}
```

***

## Execution Pipeline

### Step-by-Step Workflow

1. **Phase 0 Execution** (1 hour estimated):
   ```bash
   cargo run --release -- --experiment phase0 --instances all --runs 200
   ```
   Output: Temperature calibration files

2. **Parameter Injection**:
   ```bash
   python scripts/inject_calibrated_temps.py
   ```
   Reads Phase 0 outputs, updates EXP-01B/D/02/04B configs

3. **Phase 1-4 Execution** (12 hours estimated):
   ```bash
   for phase in {1..4}; do
     cargo run --release -- --experiment phase$phase --instances all --runs 200
   done
   ```

4. **Data Aggregation**:
   ```bash
   python scripts/aggregate_results.py --output master_results.csv
   ```

5. **LaTeX Asset Generation**:
   ```bash
   python scripts/generate_plots.py --format pgf --output figures/
   python scripts/generate_tables.py --format tex --output tables/
   ```

### Quality Assurance

**Validation Checks**:
- [ ] All 200 runs completed per config
- [ ] No NaN/Inf values in fitness columns
- [ ] Feasibility flag matches violation count
- [ ] Temperature never exceeds initial value
- [ ] CSV files parse without errors in LaTeX

***

## Expected Outcomes

1. **Neighborhood Ranking**: Clear ordering of Swap < 2-opt < Alternating < Bandit (hypothesis)
2. **Weight Thresholds**: Critical violation weight ≥ 5.0 for `inst_concours` feasibility
3. **Warmup ROI**: 2-5% improvement in best fitness, 10-20% reduction in variance
4. **Instance Characterization**: Identification of "easy" (inst1) vs. "hard" (concours) based on convergence profiles

This specification provides a publication-ready experimental framework with statistical rigor, fair comparisons, and seamless LaTeX integration for your TSPTW metaheuristics report.