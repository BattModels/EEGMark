#!/bin/bash
# Runs the HPC Challenge Benchmark and computes a singular score
# based on the GEOMEAN weight of its components (HPL, RANDOM, STREAM, FFT)
rm -f hpccoutf.txt
N_CPUS=$(python3 -c 'import os; print(int(os.cpu_count()/2))')
mpirun -n "$N_CPUS" hpcc
python3 score.py
