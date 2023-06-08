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

module load anaconda3

./run.sh