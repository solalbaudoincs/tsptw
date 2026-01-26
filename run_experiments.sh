#!/usr/bin/env bash
set -euo pipefail

HELP_MSG="
# ============================================================================
# TSPTW Simulated Annealing -- Full Experiment Pipeline
# ============================================================================
#
# Usage:
#   ./run_experiments.sh [--runs 200] [--output-dir results] [--instances all]
#
# Runs all 5 phases sequentially, injects calibrated temperatures between
# Phase 0 and Phases 1-4, then aggregates results and generates plots/tables.
# ============================================================================
"
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    echo "$HELP_MSG"
    exit 0
fi
RUNS=20
OUTPUT_DIR="results"
INSTANCES="all"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --runs)       RUNS="$2";       shift 2 ;;
        --output-dir) OUTPUT_DIR="$2"; shift 2 ;;
        --instances)  INSTANCES="$2";  shift 2 ;;
        *)            echo "Unknown arg: $1"; exit 1 ;;
    esac
done

CALIBRATED_TEMPS="$OUTPUT_DIR/calibrated_temps.json"
FIGURES_DIR="figures"
TABLES_DIR="tables"
START_TIME=$(date +%s)
echo "========================================"
echo " TSPTW Experiments"
echo "========================================"
echo " Runs per config : $RUNS"
echo " Output dir      : $OUTPUT_DIR"
echo " Instances       : $INSTANCES"
echo "========================================"
echo ""

# ---- Build ----------------------------------------------------------------
echo "[0/7] Building release binary..."
cargo build --release
echo ""

# ---- Phase 0 : Temperature Calibration -----------------------------------
echo "[1/7] Phase 0: Temperature Calibration"
echo "----------------------------------------"
cargo run --release -- experiment \
    --phase phase0 \
    --instances "$INSTANCES" \
    --runs "$RUNS" \
    --output-dir "$OUTPUT_DIR"
echo ""

# ---- Inject calibrated temperatures --------------------------------------
echo "[2/7] Injecting calibrated temperatures"
echo "----------------------------------------"
python scripts/inject_calibrated_temps.py \
    --results-dir "$OUTPUT_DIR" \
    --output "$CALIBRATED_TEMPS"
echo ""

# ---- Phase 1 : Neighborhood Comparison -----------------------------------
echo "[3/7] Phase 1: Neighborhood Operator Comparison (EXP-01)"
echo "----------------------------------------"
cargo run --release -- experiment \
    --phase phase1 \
    --instances "$INSTANCES" \
    --runs "$RUNS" \
    --output-dir "$OUTPUT_DIR" \
    --calibrated-temps "$CALIBRATED_TEMPS"
echo ""

# ---- Phase 2 : Advanced Neighborhoods ------------------------------------
echo "[4/7] Phase 2: Advanced Neighborhood Strategies (EXP-02)"
echo "----------------------------------------"
cargo run --release -- experiment \
    --phase phase2 \
    --instances "$INSTANCES" \
    --runs "$RUNS" \
    --output-dir "$OUTPUT_DIR" \
    --calibrated-temps "$CALIBRATED_TEMPS"
echo ""

# ---- Phase 3 : Weight Sensitivity ----------------------------------------
echo "[5/7] Phase 3: Weight Sensitivity Analysis (EXP-03)"
echo "----------------------------------------"
cargo run --release -- experiment \
    --phase phase3 \
    --instances "$INSTANCES" \
    --runs "$RUNS" \
    --output-dir "$OUTPUT_DIR"
echo ""

# ---- Phase 4 : Warmup Ablation -------------------------------------------
echo "[6/7] Phase 4: Warmup Ablation Study (EXP-04)"
echo "----------------------------------------"
cargo run --release -- experiment \
    --phase phase4 \
    --instances "$INSTANCES" \
    --runs "$RUNS" \
    --output-dir "$OUTPUT_DIR" \
    --calibrated-temps "$CALIBRATED_TEMPS"
echo ""

# ---- Post-processing -----------------------------------------------------
echo "[7/7] Post-processing: aggregation, plots, tables"
echo "----------------------------------------"

echo "  Aggregating results..."
python scripts/aggregate_results.py \
    --results-dir "$OUTPUT_DIR" \
    --output "$OUTPUT_DIR/master_results.csv"

echo ""
echo "  Generating plots..."
python scripts/generate_plots.py \
    --results-dir "$OUTPUT_DIR" \
    --format pgf \
    --output "$FIGURES_DIR/"

echo ""
echo "  Generating LaTeX tables..."
python scripts/generate_tables.py \
    --results-dir "$OUTPUT_DIR" \
    --format tex \
    --output "$TABLES_DIR/"

echo ""
echo "========================================"
echo " Pipeline complete"
echo "========================================"
echo " Results   : $OUTPUT_DIR/"
echo " Master CSV: $OUTPUT_DIR/master_results.csv"
echo " Figures   : $FIGURES_DIR/"
echo " Tables    : $TABLES_DIR/"
echo "========================================"
END_TIME=$(date +%s)
TOTAL_TIME=$((END_TIME - START_TIME))
echo "All phases completed successfully in $TOTAL_TIME seconds."

echo "to generate pdfs use the following command:"
echo "python scripts/generate_plots.py --results-dir results --format pdf --output figures"
