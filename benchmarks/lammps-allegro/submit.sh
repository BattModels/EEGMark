#!/usr/bin/bash

#SBATCH -t 00-01:00
#SBATCH -J lammps_benchmark
#SBATCH -o logs/output.%j
#SBATCH -e logs/error.%j
#SBATCH -p GPU-shared
#SBATCH -N 1
#SBATCH --ntasks=1
#SBATCH --gpus=v100-32:1
#SBATCH --mem-per-gpu=50000
#SBATCH --mail-user=eannevel@andrew.cmu.edu

set -e; set -o pipefail

# Load required modules
# module load singularity

# Build SIF, if it doesn't exist
if [[ ! -f lammps.sif ]]; then
    singularity build lammps.sif docker://nvcr.io/hpc/lammps:29Oct2020
fi

readonly gpus_per_node=$(( SLURM_NTASKS / SLURM_JOB_NUM_NODES  ))

echo "Running Lennard Jones 8x8x8 example on ${SLURM_NTASKS} GPUS..."
# echo "Running 2NN MEAM example on ${SLURM_NTASKS} GPUS..."
srun --mpi=pmi2 \
singularity run --nv -B ${PWD}:/host_pwd  lammps.sif \
lmp -k on g ${gpus_per_node} -sf kk -pk kokkos cuda/aware on neigh full comm device binsize 2.8 -var x 8 -var y 8 -var z 8 -in /host_pwd/run.LiMg2nnmeam.in
# lmp -k on g ${gpus_per_node} -sf kk -pk kokkos cuda/aware on neigh full comm device binsize 2.8 -var x 8 -var y 8 -var z 8 -in /host_pwd/in.lj.txt