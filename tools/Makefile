VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip

CSV = ../2025/runtimes.csv
OUTPUT = plot.png
README = ../README.md

$(VENV)/bin/python:
	python3 -m venv $(VENV)

install: $(VENV)/bin/python
	$(PIP) install pandas matplotlib tabulate

plot:
	$(PYTHON) runtime_plot.py $(CSV) $(OUTPUT) all

table:
	$(PYTHON) runtime_table.py $(CSV)

clean:
	rm -rf $(VENV)
	rm -f *.png *.svg
