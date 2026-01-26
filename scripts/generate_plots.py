#!/usr/bin/env python3
"""
Generate all PGF/PDF plots for the TSPTW SA experiment report.
Optimized with pandas and vectorized operations + aggressive downsampling.
"""

import argparse
import os
import pandas as pd
import numpy as np
import matplotlib
from scipy.stats import gaussian_kde

matplotlib.use('pgf')
import matplotlib.pyplot as plt

matplotlib.rcParams.update({
    'pgf.texsystem': 'pdflatex',
    'font.family': 'serif',
    'text.usetex': True,
    'pgf.rcfonts': False,
    'font.size': 10,
    'axes.labelsize': 10,
    'legend.fontsize': 8,
    'xtick.labelsize': 8,
    'ytick.labelsize': 8,
})

INSTANCES = ['inst1', 'inst2', 'inst3', 'inst_concours']
INSTANCE_LABELS = {
    'inst1': 'Instance 1',
    'inst2': 'Instance 2',
    'inst3': 'Instance 3',
    'inst_concours': 'Competition',
}

MAX_PLOT_POINTS = 10000000000000000000000

def save_fig(fig, path, fmt):
    if fmt == 'pgf':
        fig.savefig(path, bbox_inches='tight')
    elif fmt == 'pdf':
        fig.savefig(path, bbox_inches='tight', format='pdf')
    else:
        fig.savefig(path, bbox_inches='tight', dpi=150)
    name = os.path.basename(path)
    print(f'Saved figure {name}')
    plt.close(fig)

def downsample(df, limit=MAX_PLOT_POINTS):
    if len(df) <= limit:
        return df
    step = len(df) // limit
    return df.iloc[::step]

def plot_phase0_temp_histogram(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'phase0')
    for inst in INSTANCES:
        path = os.path.join(phase_dir, f'{inst}_phase0-Swap_warmup_temp_final.csv')
        if not os.path.isfile(path): continue
        df = pd.read_csv(path)
        vals = df['final_temp'].values
        fig, ax = plt.subplots(figsize=(5, 3.5))
        ax.hist(vals, bins=25, density=True, alpha=0.6, color='steelblue', edgecolor='white')
        if len(vals) > 1:
            kde = gaussian_kde(vals)
            xs = np.linspace(vals.min()*0.9, vals.max()*1.1, 100)
            ax.plot(xs, kde(xs), color='darkred', lw=1.5)
        ax.set_xlabel(r'Final Temp $T$')
        ax.set_title(f'{INSTANCE_LABELS[inst]} Warmup')
        ax.grid(True, alpha=0.2)
        save_fig(fig, os.path.join(output_dir, f'{inst}_temp_hist.{fmt}'), fmt)

def plot_phase0_temp_convergence(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'phase0')
    for inst in INSTANCES:
        path = os.path.join(phase_dir, f'{inst}_phase0-Swap_temp_trajectory.csv')
        if not os.path.isfile(path): continue
        df = pd.read_csv(path, usecols=['step', 'temperature'])
        st = df.groupby('step')['temperature'].agg(['mean', lambda x: np.percentile(x,25), lambda x: np.percentile(x,75)])
        st.columns = ['m','q1','q3']
        st = downsample(st)
        fig, ax = plt.subplots(figsize=(5, 3.5))
        ax.fill_between(st.index, st['q1'], st['q3'], alpha=0.2, color='tab:blue')
        ax.plot(st.index, st['m'], color='tab:blue', lw=1.5)
        ax.set_xlabel('Step'); ax.set_ylabel('Temp')
        ax.set_title(f'{INSTANCE_LABELS[inst]} Temp Conv')
        save_fig(fig, os.path.join(output_dir, f'{inst}_temp_conv.{fmt}'), fmt)

def plot_phase1_convergence(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'exp01')
    configs = [('EXP-01A','Swap+W','tab:blue'),('EXP-01B','Swap+C','tab:cyan'),
               ('EXP-01C','2opt+W','tab:red'),('EXP-01D','2opt+C','tab:orange')]
    for inst in INSTANCES:
        fig, ax = plt.subplots(figsize=(6, 4))
        for cid, lbl, col in configs:
            path = os.path.join(phase_dir, f'{inst}_{cid}_convergence.csv')
            if os.path.isfile(path):
                df = downsample(pd.read_csv(path))
                ax.fill_between(df['step'], df['q1_fitness'], df['q3_fitness'], alpha=0.1, color=col)
                ax.plot(df['step'], df['median_fitness'], color=col, lw=1.2, label=lbl)

        ax.set_title(f'{INSTANCE_LABELS[inst]} Neighborhoods')
        ax.legend(fontsize=7); ax.grid(True, alpha=0.2)
        save_fig(fig, os.path.join(output_dir, f'{inst}_exp01_conv.{fmt}'), fmt)

def plot_phase1_boxplot(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'exp01')
    cfg = ['EXP-01A','EXP-01B','EXP-01C','EXP-01D']
    fig, axes = plt.subplots(1, len(INSTANCES), figsize=(12, 4))
    for ax, inst in zip(axes, INSTANCES):
        data = []
        for cid in cfg:
            p = os.path.join(phase_dir, f'{inst}_{cid}_runs.csv')
            data.append(pd.read_csv(p)['best_fitness'].values)
        ax.boxplot(data, labels=['S+W','S+C','2+W','2+C'], patch_artist=True, showfliers=True)
        ax.set_title(inst)
    save_fig(fig, os.path.join(output_dir, f'exp01_boxplot.{fmt}'), fmt)

def plot_phase2_bandit_dynamics(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'exp02')
    for inst in INSTANCES:
        p = os.path.join(phase_dir, f'{inst}_EXP-02B_bandit_stats.csv')
        if not os.path.isfile(p): continue
        df = pd.read_csv(p)
        agg = downsample(df.groupby('step').agg({
            'swap_selections':'mean','twoopt_selections':'mean',
            'swap_avg_reward':['mean','std','count'],'twoopt_avg_reward':['mean','std','count']
        }))
        s = agg.index; sw = agg['swap_selections']['mean']; to = agg['twoopt_selections']['mean']
        tot = np.maximum(sw+to, 1)
        fig, (ax1,ax2) = plt.subplots(2,1,figsize=(6,6), sharex=True)
        ax1.stackplot(s, sw/tot, to/tot, labels=['Swap','2-opt'], colors=['tab:blue','tab:red'], alpha=0.6)
        ax1.legend(loc='lower left')
        for c, cl, lb in [('swap_avg_reward','tab:blue','Swap'),('twoopt_avg_reward','tab:red','2-opt')]:
            m = agg[c]['mean']; ci = 1.96*agg[c]['std']/np.sqrt(np.maximum(agg[c]['count'],1))
            ax2.plot(s, m, color=cl, label=lb); ax2.fill_between(s, m-ci, m+ci, color=cl, alpha=0.1)
        ax2.legend(); ax1.set_title(f'{inst} Bandit')
        save_fig(fig, os.path.join(output_dir, f'{inst}_bandit.{fmt}'), fmt)

def plot_phase3_pareto(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'exp03')
    cfgs = [('EXP-03A','Base','tab:blue'),('EXP-03B','Hard','tab:red'),('EXP-03C','Relax','tab:green'),('EXP-03D','Dist','tab:purple')]
    for inst in INSTANCES:
        fig, ax = plt.subplots(figsize=(5,4))
        for cid, lbl, col in cfgs:
            p = os.path.join(phase_dir, f'{inst}_{cid}_runs.csv')
            df = pd.read_csv(p)
            f = df['feasible'].astype(str).str.lower().isin(['true','1','1.0'])
            ax.scatter(df[f]['total_distance'], df[f]['violations'], color=col, marker='o', s=10, alpha=0.5, label=lbl)
            ax.scatter(df[~f]['total_distance'], df[~f]['violations'], color=col, marker='x', s=10, alpha=0.5)
        ax.legend(fontsize=7)
        ax.set_title(f'{inst} Pareto')
        save_fig(fig, os.path.join(output_dir, f'{inst}_pareto.{fmt}'), fmt)

def plot_phase4_convergence(results_dir, output_dir, fmt):
    phase_dir = os.path.join(results_dir, 'exp04')
    for inst in INSTANCES:
        fig, ax = plt.subplots(figsize=(6,4))
        for cid, lbl, col in [('EXP-04A','Warmup','tab:blue'),('EXP-04B','Cold','tab:red')]:
            p = os.path.join(phase_dir, f'{inst}_{cid}_convergence.csv')
            if os.path.isfile(p):
                df = downsample(pd.read_csv(p))
                ax.fill_between(df['step'], df['q1_fitness'], df['q3_fitness'], alpha=0.15, color=col)
                ax.plot(df['step'], df['median_fitness'], color=col, label=lbl)
        ax.legend()
        ax.set_title(f'{inst} Warmup Ablation')
        save_fig(fig, os.path.join(output_dir, f'{inst}_exp04_conv.{fmt}'), fmt)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--results-dir', default='results')
    parser.add_argument('--format', default='pgf')
    parser.add_argument('--output', default='figures/')
    args = parser.parse_args()
    os.makedirs(args.output, exist_ok=True)
    fmt = args.format
    import multiprocessing

    
    with multiprocessing.Pool() as pool: # we got no time to waste
        pool.starmap(plot_phase1_boxplot, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase1_convergence, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase0_temp_histogram, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase0_temp_convergence, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase2_bandit_dynamics, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase3_pareto, [(args.results_dir, args.output, fmt)])
        pool.starmap(plot_phase4_convergence, [(args.results_dir, args.output, fmt)])

if __name__ == '__main__':
    main()
