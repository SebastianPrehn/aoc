VENV = .venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip

CSV = ../2025/runtimes.csv
OUTPUT = plot.png

$(VENV)/bin/python:
	python3 -m venv $(VENV)

install: $(VENV)/bin/python
	$(PIP) install pandas matplotlib

plot:
	$(PYTHON) runtime_plot.py $(CSV) $(OUTPUT)

clean:
	rm -rf $(VENV)
	rm -f *.png *.svg
