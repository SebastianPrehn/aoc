import pandas as pd
import sys, os, re

START = "<!-- START CSV TABLE -->"
END = "<!-- END CSV TABLE -->"

def update_readme(csv_path, readme_path='../README.md', marker='<!-- CSV_TABLE_MARKER -->'):
    if not os.path.exists(csv_path):
        print(f"Error: File not found: {csv_path}")
        return

    df = pd.read_csv(csv_path)
    df.rename(columns={
    "day": "Day",
    "part1_ms": "Part 1 Runtime",
    "part2_ms": "Part 2 Runtime"
    }, inplace=True)

    # Convert Day to Markdown links
    df["Day"] = df["Day"].apply(lambda d: f"[{d}](2025/day{d:02d}/src/main.rs)")

    # Append 'ms' to runtimes and format
    for col in ["Part 1 Runtime", "Part 2 Runtime"]:
        df[col] = df[col].apply(lambda x: f"{x:.6f}ms")
    
    md_table = df.to_markdown(index=False)
    table_block = f"{START}\n{md_table}\n{END}"

    if os.path.exists(readme_path):
        with open(readme_path, 'r') as f:
            readme = f.read()
    else:
        readme = ''

    # Replace existing table block if present
    pattern = r'<!-- START CSV TABLE -->.*?<!-- END CSV TABLE -->'
    if re.search(pattern, readme, flags=re.DOTALL):
        new_content = re.sub(pattern, table_block, readme, flags=re.DOTALL)
    # If no table exists yet, insert at marker
    elif marker in readme:
        new_content = readme.replace(marker, f"{marker}\n{table_block}")
    # Otherwise append to bottom
    else:
        new_content = readme + "\n\n" + marker + "\n" + table_block

    with open(readme_path, 'w') as f:
        f.write(new_content)

    print(f"README.md updated with CSV table from {csv_path}.")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print(" python runtime_table.py path/to/runtime.csv [README.md]")
    else:
        csv_path = sys.argv[1]
        readme_path = sys.argv[2] if len(sys.argv) >= 3 else '../README.md'
        update_readme(csv_path, readme_path)
