#!/usr/bin/env python3
"""
Generate LaTeX tables for the TSPTW SA experiment report.

Outputs:
  - tables/phase0_calibration.tex    -- Temperature calibration summary
  - tables/exp01_statistics.tex      -- Wilcoxon tests, convergence speed, CV
  - tables/exp03_feasibility.tex     -- Feasibility rate table
  - tables/exp04_warmup.tex          -- Paired t-test, Cohen's d, warmup efficiency
  - tables/master_results.tex        -- Consolidated summary table

Usage:
    python scripts/generate_tables.py [--results-dir results] [--format tex] [--output tables/]
"""

import argparse
import csv
import json
import os
import sys
from collections import defaultdict

import numpy as np
from scipy import stats as sp_stats


INSTANCES = ["inst1", "inst2", "inst3", "inst_concours"]
INSTANCE_SHORT = {
    "inst1": "inst1",
    "inst2": "inst2",
    "inst3": "inst3",
    "inst_concours": "concours",
}


def load_csv(path):
    """Load CSV file into list of dicts with numeric conversion."""
    rows = []
    with open(path, newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            converted = {}
            for k, v in row.items():
                try:
                    converted[k] = float(v)
                except (ValueError, TypeError):
                    converted[k] = v
            rows.append(converted)
    return rows


def write_tex(path, content):
    """Write LaTeX content to file."""
    with open(path, "w") as f:
        f.write(content)
    print(f"  Saved {path}")


# ---------------------------------------------------------------------------
# Phase 0: Temperature Calibration Summary
# ---------------------------------------------------------------------------

def generate_phase0_table(results_dir, output_dir):
    """Statistical summary of warmup temperatures per instance."""
    phase_dir = os.path.join(results_dir, "phase0")
    if not os.path.isdir(phase_dir):
        print("Skipping Phase 0 table: directory not found")
        return

    rows_tex = []
    for inst in INSTANCES:
        stats_path = os.path.join(phase_dir, f"{inst}_phase0-Swap_warmup_stats.json")
        if not os.path.isfile(stats_path):
            continue

        with open(stats_path) as f:
            s = json.load(f)

        cv = s["std"] / s["mean"] if s["mean"] > 0 else float("inf")
        flag = r" $\dagger$" if cv > 0.3 else ""

        rows_tex.append(
            f"  {INSTANCE_SHORT[inst]} & "
            f"{s['mean']:.1f} & {s['std']:.1f} & "
            f"{s['median']:.1f} & "
            f"[{s['q1']:.1f}, {s['q3']:.1f}] & "
            f"{cv:.3f}{flag} \\\\"
        )

    content = r"""\begin{tabular}{lccccc}
\toprule
Instance & $\mu(T)$ & $\sigma(T)$ & Median & IQR & CV \\
\midrule
""" + "\n".join(rows_tex) + r"""
\bottomrule
\end{tabular}
"""

    write_tex(os.path.join(output_dir, "phase0_calibration.tex"), content)


# ---------------------------------------------------------------------------
# Phase 1: Statistical Analysis
# ---------------------------------------------------------------------------

def generate_phase1_table(results_dir, output_dir):
    """Wilcoxon signed-rank test, convergence speed, robustness CV."""
    phase_dir = os.path.join(results_dir, "exp01")
    if not os.path.isdir(phase_dir):
        print("Skipping Phase 1 table: directory not found")
        return

    # --- Wilcoxon test: Swap vs 2-opt (paired by run seed) ---
    wilcoxon_rows = []
    for inst in INSTANCES:
        pairs = {}
        for warmup_label, swap_id, twoopt_id in [
            ("Warmup", "EXP-01A", "EXP-01C"),
            ("Cold", "EXP-01B", "EXP-01D"),
        ]:
            swap_path = os.path.join(phase_dir, f"{inst}_{swap_id}_runs.csv")
            twoopt_path = os.path.join(phase_dir, f"{inst}_{twoopt_id}_runs.csv")
            if not os.path.isfile(swap_path) or not os.path.isfile(twoopt_path):
                continue

            swap_rows = load_csv(swap_path)
            twoopt_rows = load_csv(twoopt_path)

            swap_by_run = {int(r["run_id"]): r["best_fitness"] for r in swap_rows}
            twoopt_by_run = {int(r["run_id"]): r["best_fitness"] for r in twoopt_rows}

            common_runs = sorted(set(swap_by_run.keys()) & set(twoopt_by_run.keys()))
            if len(common_runs) < 2:
                continue

            swap_vals = [swap_by_run[r] for r in common_runs]
            twoopt_vals = [twoopt_by_run[r] for r in common_runs]

            stat, p_value = sp_stats.wilcoxon(swap_vals, twoopt_vals)
            sig = significance_stars(p_value)

            wilcoxon_rows.append(
                f"  {INSTANCE_SHORT[inst]} & {warmup_label} & "
                f"{np.mean(swap_vals):.1f} & {np.mean(twoopt_vals):.1f} & "
                f"{stat:.0f} & {p_value:.4f} & {sig} \\\\"
            )

    # --- Robustness: CV of best fitness ---
    cv_rows = []
    for inst in INSTANCES:
        cv_data = []
        for config_id in ["EXP-01A", "EXP-01B", "EXP-01C", "EXP-01D"]:
            runs_path = os.path.join(phase_dir, f"{inst}_{config_id}_runs.csv")
            if not os.path.isfile(runs_path):
                continue
            rows = load_csv(runs_path)
            fitnesses = [r["best_fitness"] for r in rows]
            mean_f = np.mean(fitnesses)
            std_f = np.std(fitnesses)
            cv = std_f / mean_f if mean_f > 0 else 0
            cv_data.append(f"{cv:.3f}")

        if cv_data:
            cv_rows.append(
                f"  {INSTANCE_SHORT[inst]} & " + " & ".join(cv_data) + r" \\"
            )

    # --- Convergence speed: step to reach 90% of best-ever ---
    speed_rows = []
    for inst in INSTANCES:
        speed_data = []
        for config_id in ["EXP-01A", "EXP-01B", "EXP-01C", "EXP-01D"]:
            conv_path = os.path.join(phase_dir, f"{inst}_{config_id}_convergence.csv")
            if not os.path.isfile(conv_path):
                speed_data.append("--")
                continue
            rows = load_csv(conv_path)
            if not rows:
                speed_data.append("--")
                continue

            final_fitness = rows[-1]["median_fitness"]
            initial_fitness = rows[0]["median_fitness"]
            threshold = initial_fitness - 0.9 * (initial_fitness - final_fitness)

            found_step = "--"
            for r in rows:
                if r["median_fitness"] <= threshold:
                    found_step = f"{int(r['step']):,}"
                    break
            speed_data.append(found_step)

        speed_rows.append(
            f"  {INSTANCE_SHORT[inst]} & " + " & ".join(speed_data) + r" \\"
        )

    content = r"""% --- Wilcoxon Signed-Rank Test: Swap vs 2-opt ---
\begin{tabular}{llccccl}
\toprule
Instance & Init & $\bar{F}_{\text{Swap}}$ & $\bar{F}_{\text{2-opt}}$ & $W$ & $p$ & Sig. \\
\midrule
""" + "\n".join(wilcoxon_rows) + r"""
\bottomrule
\end{tabular}

\vspace{1em}

% --- Robustness: Coefficient of Variation ---
\begin{tabular}{lcccc}
\toprule
Instance & EXP-01A & EXP-01B & EXP-01C & EXP-01D \\
\midrule
""" + "\n".join(cv_rows) + r"""
\bottomrule
\end{tabular}

\vspace{1em}

% --- Convergence Speed: Step to 90\% of best ---
\begin{tabular}{lcccc}
\toprule
Instance & EXP-01A & EXP-01B & EXP-01C & EXP-01D \\
\midrule
""" + "\n".join(speed_rows) + r"""
\bottomrule
\end{tabular}
"""

    write_tex(os.path.join(output_dir, "exp01_statistics.tex"), content)


# ---------------------------------------------------------------------------
# Phase 3: Feasibility Rate Table
# ---------------------------------------------------------------------------

def generate_phase3_table(results_dir, output_dir):
    """Feasibility rate table across weight configs and instances."""
    phase_dir = os.path.join(results_dir, "exp03")
    if not os.path.isdir(phase_dir):
        print("Skipping Phase 3 table: directory not found")
        return

    configs = [
        ("EXP-03A", "Baseline"),
        ("EXP-03B", "Hard"),
        ("EXP-03C", "Relaxed"),
        ("EXP-03D", "Distance"),
    ]

    rows_tex = []
    for config_id, label in configs:
        rates = []
        for inst in INSTANCES:
            runs_path = os.path.join(phase_dir, f"{inst}_{config_id}_runs.csv")
            if not os.path.isfile(runs_path):
                rates.append("--")
                continue
            rows = load_csv(runs_path)
            feasible_count = sum(
                1 for r in rows
                if str(r.get("feasible", "")).lower() in ("true", "1", "1.0")
            )
            rate = feasible_count / len(rows) if rows else 0
            rates.append(f"{rate:.2f}")

        rows_tex.append(f"  {label} & " + " & ".join(rates) + r" \\")

    content = r"""\begin{tabular}{lcccc}
\toprule
Config & inst1 $F_r$ & inst2 $F_r$ & inst3 $F_r$ & concours $F_r$ \\
\midrule
""" + "\n".join(rows_tex) + r"""
\bottomrule
\end{tabular}
"""

    write_tex(os.path.join(output_dir, "exp03_feasibility.tex"), content)


# ---------------------------------------------------------------------------
# Phase 4: Warmup Ablation Statistics
# ---------------------------------------------------------------------------

def generate_phase4_table(results_dir, output_dir):
    """Paired t-test, Cohen's d, warmup efficiency."""
    phase_dir = os.path.join(results_dir, "exp04")
    if not os.path.isdir(phase_dir):
        print("Skipping Phase 4 table: directory not found")
        return

    rows_tex = []
    for inst in INSTANCES:
        warmup_path = os.path.join(phase_dir, f"{inst}_EXP-04A_runs.csv")
        cold_path = os.path.join(phase_dir, f"{inst}_EXP-04B_runs.csv")
        if not os.path.isfile(warmup_path) or not os.path.isfile(cold_path):
            continue

        warmup_rows = load_csv(warmup_path)
        cold_rows = load_csv(cold_path)

        w_by_run = {int(r["run_id"]): r for r in warmup_rows}
        c_by_run = {int(r["run_id"]): r for r in cold_rows}

        common = sorted(set(w_by_run.keys()) & set(c_by_run.keys()))
        if len(common) < 2:
            continue

        w_fitness = np.array([w_by_run[r]["best_fitness"] for r in common])
        c_fitness = np.array([c_by_run[r]["best_fitness"] for r in common])

        # Paired t-test
        t_stat, p_value = sp_stats.ttest_rel(w_fitness, c_fitness)
        sig = significance_stars(p_value)

        # Cohen's d
        diff = c_fitness - w_fitness  # positive means warmup is better
        cohens_d = np.mean(diff) / np.std(diff, ddof=1) if np.std(diff, ddof=1) > 0 else 0

        # Warmup efficiency
        w_times = np.array([w_by_run[r]["execution_time_ms"] for r in common])
        c_times = np.array([c_by_run[r]["execution_time_ms"] for r in common])
        time_overhead = np.mean(w_times) - np.mean(c_times)
        fitness_improvement = np.mean(c_fitness) - np.mean(w_fitness)
        efficiency = fitness_improvement / time_overhead if time_overhead > 0 else 0

        rows_tex.append(
            f"  {INSTANCE_SHORT[inst]} & "
            f"{np.mean(w_fitness):.1f} & {np.mean(c_fitness):.1f} & "
            f"{t_stat:.2f} & {p_value:.4f} & {sig} & "
            f"{cohens_d:.3f} & {efficiency:.4f} \\\\"
        )

    content = r"""\begin{tabular}{lcccclcc}
\toprule
Instance & $\bar{F}_{\text{warmup}}$ & $\bar{F}_{\text{cold}}$ & $t$ & $p$ & Sig. & Cohen's $d$ & $\eta_{\text{warmup}}$ \\
\midrule
""" + "\n".join(rows_tex) + r"""
\bottomrule
\end{tabular}
"""

    write_tex(os.path.join(output_dir, "exp04_warmup.tex"), content)


# ---------------------------------------------------------------------------
# Master Results Table
# ---------------------------------------------------------------------------

def generate_master_table(results_dir, output_dir):
    """Consolidated summary table across all phases."""
    phase_map = {
        "0": ("phase0", ["phase0-Swap", "phase0-TwoOpt"]),
        "1": ("exp01", ["EXP-01A", "EXP-01B", "EXP-01C", "EXP-01D"]),
        "2": ("exp02", ["EXP-02A", "EXP-02B"]),
        "3": ("exp03", ["EXP-03A", "EXP-03B", "EXP-03C", "EXP-03D"]),
        "4": ("exp04", ["EXP-04A", "EXP-04B"]),
    }

    rows_tex = []
    for phase_num, (phase_subdir, config_ids) in phase_map.items():
        phase_dir = os.path.join(results_dir, phase_subdir)
        if not os.path.isdir(phase_dir):
            continue

        for config_id in config_ids:
            for inst in INSTANCES:
                runs_path = os.path.join(phase_dir, f"{inst}_{config_id}_runs.csv")
                if not os.path.isfile(runs_path):
                    continue

                rows = load_csv(runs_path)
                if not rows:
                    continue

                fitnesses = [r["best_fitness"] for r in rows]
                times_ms = [r["execution_time_ms"] for r in rows]
                feasible = [
                    str(r.get("feasible", "")).lower() in ("true", "1", "1.0")
                    for r in rows
                ]

                best = np.min(fitnesses)
                mean = np.mean(fitnesses)
                std = np.std(fitnesses)
                fr = sum(feasible) / len(feasible)
                time_s = np.mean(times_ms) / 1000.0

                # For Phase 0, fitness metrics are less meaningful
                if phase_num == "0":
                    rows_tex.append(
                        f"  {phase_num} & {config_id} & {INSTANCE_SHORT[inst]} & "
                        f"-- & -- & -- & -- & {time_s:.1f} \\\\"
                    )
                else:
                    rows_tex.append(
                        f"  {phase_num} & {config_id} & {INSTANCE_SHORT[inst]} & "
                        f"{best:.1f} & {mean:.1f} & {std:.1f} & "
                        f"{fr:.2f} & {time_s:.1f} \\\\"
                    )

    content = r"""\begin{tabular}{llcccccr}
\toprule
Phase & Config & Instance & Best & Mean & Std & $F_r$ & Time (s) \\
\midrule
""" + "\n".join(rows_tex) + r"""
\bottomrule
\end{tabular}
"""

    write_tex(os.path.join(output_dir, "master_results.tex"), content)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def significance_stars(p):
    """Return significance stars for p-value."""
    if p < 0.001:
        return "***"
    elif p < 0.01:
        return "**"
    elif p < 0.05:
        return "*"
    else:
        return "n.s."


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Generate LaTeX tables")
    parser.add_argument("--results-dir", default="results", help="Root results directory")
    parser.add_argument("--format", default="tex", help="Output format (tex)")
    parser.add_argument("--output", default="tables/", help="Output directory")
    args = parser.parse_args()

    os.makedirs(args.output, exist_ok=True)

    print("Phase 0: Calibration Summary Table")
    generate_phase0_table(args.results_dir, args.output)

    print("\nPhase 1: Statistical Analysis Tables")
    generate_phase1_table(args.results_dir, args.output)

    print("\nPhase 3: Feasibility Rate Table")
    generate_phase3_table(args.results_dir, args.output)

    print("\nPhase 4: Warmup Ablation Statistics")
    generate_phase4_table(args.results_dir, args.output)

    print("\nMaster Results Table")
    generate_master_table(args.results_dir, args.output)

    print(f"\nAll tables saved to {args.output}")


if __name__ == "__main__":
    main()
