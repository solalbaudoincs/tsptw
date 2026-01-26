#!/usr/bin/env python3
"""
Aggregate all experiment CSV results into a master_results.csv with
per-config summary statistics.

Usage:
    python scripts/aggregate_results.py [--results-dir results] [--output master_results.csv]
"""

import argparse
import csv
import json
import os
import sys
from collections import defaultdict

import numpy as np


INSTANCES = ["inst1", "inst2", "inst3", "inst_concours"]

PHASE_DIRS = {
    "phase0": "phase0",
    "phase1": "exp01",
    "phase2": "exp02",
    "phase3": "exp03",
    "phase4": "exp04",
}

PHASE_CONFIGS = {
    "phase0": ["phase0-Swap", "phase0-TwoOpt"],
    "phase1": ["EXP-01A", "EXP-01B", "EXP-01C", "EXP-01D"],
    "phase2": ["EXP-02A", "EXP-02B"],
    "phase3": ["EXP-03A", "EXP-03B", "EXP-03C", "EXP-03D"],
    "phase4": ["EXP-04A", "EXP-04B"],
}


def load_runs_csv(path):
    """Load a runs CSV file and return list of dicts."""
    rows = []
    with open(path, newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append(row)
    return rows


def compute_summary(rows):
    """Compute summary statistics from run result rows."""
    best_fitnesses = [float(r["best_fitness"]) for r in rows]
    exec_times = [float(r["execution_time_ms"]) for r in rows]
    feasible_flags = [r["feasible"].lower() in ("true", "1") for r in rows]

    arr = np.array(best_fitnesses)

    return {
        "num_runs": len(rows),
        "best": float(np.min(arr)),
        "mean": float(np.mean(arr)),
        "std": float(np.std(arr)),
        "median": float(np.median(arr)),
        "q1": float(np.percentile(arr, 25)),
        "q3": float(np.percentile(arr, 75)),
        "feasible_rate": sum(feasible_flags) / len(feasible_flags) if feasible_flags else 0.0,
        "mean_time_s": float(np.mean(exec_times)) / 1000.0,
    }


def main():
    parser = argparse.ArgumentParser(description="Aggregate experiment results")
    parser.add_argument("--results-dir", default="results", help="Root results directory")
    parser.add_argument("--output", default="master_results.csv", help="Output CSV path")
    args = parser.parse_args()

    results = []
    missing = []

    for phase, phase_dir_name in PHASE_DIRS.items():
        phase_dir = os.path.join(args.results_dir, phase_dir_name)
        if not os.path.isdir(phase_dir):
            print(f"Skipping {phase}: directory not found ({phase_dir})")
            continue

        configs = PHASE_CONFIGS[phase]
        for config_id in configs:
            for inst in INSTANCES:
                runs_path = os.path.join(phase_dir, f"{inst}_{config_id}_runs.csv")
                if not os.path.isfile(runs_path):
                    missing.append(runs_path)
                    continue

                rows = load_runs_csv(runs_path)
                if not rows:
                    missing.append(runs_path + " (empty)")
                    continue

                summary = compute_summary(rows)

                # Phase number for the table
                phase_num = phase.replace("phase", "")

                results.append({
                    "phase": phase_num,
                    "config": config_id,
                    "instance": inst,
                    "num_runs": summary["num_runs"],
                    "best": f"{summary['best']:.1f}",
                    "mean": f"{summary['mean']:.1f}",
                    "std": f"{summary['std']:.1f}",
                    "median": f"{summary['median']:.1f}",
                    "q1": f"{summary['q1']:.1f}",
                    "q3": f"{summary['q3']:.1f}",
                    "feasible_rate": f"{summary['feasible_rate']:.3f}",
                    "mean_time_s": f"{summary['mean_time_s']:.1f}",
                })

    # Validation checks
    print(f"\nProcessed {len(results)} config-instance combinations")
    if missing:
        print(f"\nMissing files ({len(missing)}):")
        for m in missing:
            print(f"  - {m}")

    # Check for NaN/Inf
    nan_count = 0
    for r in results:
        for k in ["best", "mean", "std"]:
            val = float(r[k])
            if np.isnan(val) or np.isinf(val):
                nan_count += 1
                print(f"WARNING: {k}={r[k]} for {r['config']}/{r['instance']}")
    if nan_count == 0:
        print("Validation: No NaN/Inf values found")

    # Write master CSV
    fieldnames = ["phase", "config", "instance", "num_runs", "best", "mean", "std",
                  "median", "q1", "q3", "feasible_rate", "mean_time_s"]
    with open(args.output, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(results)

    print(f"\nMaster results written to {args.output}")


if __name__ == "__main__":
    main()
