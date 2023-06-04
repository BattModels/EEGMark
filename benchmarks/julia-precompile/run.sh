#!/bin/bash
source activate

# Clean out existing pre-compile cache
rm -rf ~/.julia/compiled

# Setup for benchmarking run
export JULIA_NUM_PRECOMPILE_TASKS=$(python3 -c 'import os; print(os.cpu_count())')
export JULIA_PKG_PRECOMPILE_AUTO=0
julia +release --startup-file=no --project -- benchmark_precompile.jl
