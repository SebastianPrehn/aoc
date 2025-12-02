import pandas as pd
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

import sys, os

def plot_runtimes(csv_path, save_path=None):
    if not os.path.exists(csv_path):
        print(f"Error: File not found: {csv_path}")
        return

    df = pd.read_csv(csv_path)

    plt.figure(figsize=(8, 4))
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

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print(" python runtime_plot.py path/to/runtimes.csv [output.svg|output.png]")
    else:
        csv_path = sys.argv[1]
        save_path = sys.argv[2] if len(sys.argv) == 3 else None
        plot_runtimes(csv_path, save_path)
