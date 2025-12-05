import pandas as pd
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

import sys, os
import random
import numpy as np

def plot_line(df, save_path=None):
    plt.figure(figsize=(8,4))
    plt.plot(df["day"], df["part1_ms"], marker="o", label="Part 1 runtime (ms)")
    plt.plot(df["day"], df["part2_ms"], marker="o", label="Part 2 runtime (ms)")

    plt.xlabel("Day")
    plt.ylabel("Runtime (ms)")
    plt.title("Advent of Code - Solution Runtimes Per Day")
    plt.xticks(df["day"])
    plt.yscale("log")
    plt.grid(True, linestyle="--", alpha=0.5)
    plt.legend()
    plt.tight_layout()

    if save_path:
        plt.savefig(save_path, dpi=300)
        print(f"Saved plot to {save_path}")
    else:
        plt.show()

def plot_grouped_bar(df, save_path=None):
    days = df["day"].values
    bar_width = 0.35
    x = np.arange(len(days))

    colors = [(random.random(), random.random(), random.random())
              for _ in days]

    plt.figure(figsize=(8,4))

    plt.bar(x - bar_width/2, df["part1_ms"], 
            width=bar_width, 
            color=colors, 
            label="Part 1")

    plt.bar(x + bar_width/2, df["part2_ms"], 
            width=bar_width, 
            color=colors, 
            alpha=0.6,
            label="Part 2")

    plt.xticks(x, days)
    plt.yscale("log")
    plt.xlabel("Day")
    plt.ylabel("Runtime (ms)")
    plt.title("Advent of Code - Grouped Bar Chart")
    plt.tight_layout()

    if save_path:
        plt.savefig(save_path, dpi=300)
    else:
        plt.show()

def plot_runtimes(csv_path, save_path=None, plot_type="line"):
    if not os.path.exists(csv_path):
        print(f"Error: File not found: {csv_path}")
        return

    df = pd.read_csv(csv_path)

    if plot_type == "line":
        plot_line(df, save_path)
    elif plot_type == "bars":
        plot_grouped_bar(df, save_path)
    elif plot_type == "all":
        if save_path is None:
            plot_line(df)
            plot_grouped_bar(df)
        else:
            base, ext = os.path.splitext(save_path)
            line_path = f"{base}_line{ext}"
            bar_path = f"{base}_bars{ext}"

            print(f"Saving line plot to {line_path}")
            plot_line(df, line_path)

            print(f"Saving grouped bar plot to {bar_path}")
            plot_grouped_bar(df, bar_path)
    else:
        print(f"Unknown plot type: {plot_type}. Expected 'line' or 'bars'.")

    
if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print(" python runtime_plot.py path/to/runtimes.csv [output.svg|output.png] [line|bars|all]")
    else:
        csv_path = sys.argv[1]
        save_path = sys.argv[2] if len(sys.argv) >= 3 else None
        plot_type = sys.argv[3] if len(sys.argv) >= 4 else "line"
        plot_runtimes(csv_path, save_path, plot_type)
