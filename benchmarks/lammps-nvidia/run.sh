#!/usr/bin/bash

set -e; set -o pipefail

# Build SIF, if it doesn't exist
if [[ ! -f lammps.sif ]]; then
    singularity build lammps.sif docker://nvcr.io/hpc/lammps:29Oct2020
fi

readonly gpus_per_node=$(( SLURM_NTASKS / SLURM_JOB_NUM_NODES  ))

echo "Running Lennard Jones 8x8x8 example on ${SLURM_NTASKS} GPUS..."
# echo "Running 2NN MEAM example on ${SLURM_NTASKS} GPUS..."
srun --mpi=pmi2 \
singularity run --nv -B ${PWD}:/host_pwd  lammps.sif \
lmp -k on g ${gpus_per_node} -sf kk -pk kokkos cuda/aware on neigh full comm device binsize 2.8 -var x 8 -var y 8 -var z 8 -in /host_pwd/in.lj.txt

conda activate env
python get_performance.py