#!/usr/bin/bash

# This script assumes the singularity container "allegro.sif" is in the
# same directory and that the host has up-to-date CUDA drivers

# Run on 1 node with 1 GPU
TOTAL_GPUS=1
GPUS_PER_NODE=1
singularity exec allegro.sif mpirun -np ${TOTAL_GPUS} lmp -sf kk -k on g ${GPUS_PER_NODE} -pk kokkos newton on neigh full -in allegro_li.lammps
