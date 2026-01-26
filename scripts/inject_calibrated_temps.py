#!/usr/bin/env python3
"""
Read Phase 0 warmup stats JSON files and output a calibrated_temps.json
file for use in Phases 1-4.

Usage:
    python scripts/inject_calibrated_temps.py [--results-dir results] [--output calibrated_temps.json]
"""

import argparse
import json
import os
import sys


INSTANCES = ["inst1", "inst2", "inst3", "inst_concours"]
NEIGHBORHOODS = ["phase0-Swap", "phase0-TwoOpt"]


def main():
    parser = argparse.ArgumentParser(description="Extract calibrated temperatures from Phase 0 results")
    parser.add_argument("--results-dir", default="results", help="Root results directory")
    parser.add_argument("--output", default="calibrated_temps.json", help="Output JSON file path")
    parser.add_argument("--neighborhood", default="phase0-Swap",
                        choices=NEIGHBORHOODS,
                        help="Which neighborhood's warmup to use for calibration")
    args = parser.parse_args()

    phase0_dir = os.path.join(args.results_dir, "phase0")
    if not os.path.isdir(phase0_dir):
        print(f"Error: Phase 0 results directory not found: {phase0_dir}", file=sys.stderr)
        sys.exit(1)

    calibrated_temps = {}
    for inst in INSTANCES:
        stats_path = os.path.join(phase0_dir, f"{inst}_{args.neighborhood}_warmup_stats.json")
        if not os.path.isfile(stats_path):
            print(f"Warning: Missing warmup stats for {inst}: {stats_path}", file=sys.stderr)
            continue

        with open(stats_path) as f:
            stats = json.load(f)

        mean_temp = stats["mean"]
        std_temp = stats["std"]
        cv = std_temp / mean_temp if mean_temp > 0 else float("inf")

        calibrated_temps[inst] = mean_temp

        # Validation: flag high variance
        if cv > 0.3:
            print(f"WARNING: High variance for {inst}: CV = {cv:.3f} "
                  f"(sigma={std_temp:.1f}, mu={mean_temp:.1f}). "
                  f"Consider extended warmup duration.", file=sys.stderr)

        print(f"{inst}: mu(T) = {mean_temp:.1f}, sigma(T) = {std_temp:.1f}, "
              f"median = {stats['median']:.1f}, "
              f"IQR = [{stats['q1']:.1f}, {stats['q3']:.1f}], "
              f"CV = {cv:.3f}")

    with open(args.output, "w") as f:
        json.dump(calibrated_temps, f, indent=2)

    print(f"\nCalibrated temperatures written to {args.output}")


if __name__ == "__main__":
    main()
