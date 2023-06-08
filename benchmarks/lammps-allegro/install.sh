#!/bin/bash

# Fail on error
set -o pipefail
set -e

module load anaconda3
conda create -n env python=3.9

nproc=1

# Download Lammps
# git clone -b stable_29Sep2021_update2 --depth 1 git@github.com:lammps/lammps
git clone -b stable_23Jun2022 https://github.com/lammps/lammps.git lammps

# Download Allegro Patch
git clone https://github.com/mir-group/pair_allegro.git

# Install Allegro Patch
cd pair_allegro
./patch_lammps.sh ../lammps/
cd ../

# Download LibTorch
wget https://download.pytorch.org/libtorch/cu117/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcu117.zip
unzip libtorch-cxx11-abi-shared-with-deps-2.0.1+cu117.zip

# Install MKL

# Load OpenMPI
module load openmpi

# Load CUDA
module load cuda/11.7.1

# Install OpenMP
# spack install llvm-openmp
# spack load llvm-openmp



# Install LAMMPS
cd lammps
mkdir build
cd build
cmake ../cmake -DCMAKE_PREFIX_PATH=../../libtorch \
    -DPKG_KOKKOS=ON -DKokkos_ENABLE_CUDA=ON -DKokkos_ENABLE_OPENMP=off \
    -DCMAKE_CXX_COMPILER=/jet/home/anneveli/github/EEGMark/benchmarks/lammps-allegro/lammps/lib/kokkos/bin/nvcc_wrapper

cmake -C ../cmake/presets/basic.cmake -D BUILD_SHARED_LIBS=on -D BUILD_MPI=on \
      -D LAMMPS_EXCEPTIONS=on -D PKG_QEQ=on -D PKG_MEAM=on ../cmake

# make -j$(nproc)
# #SBATCH -A venkvis_gpu
#SBATCH --gres=gpu:1
# srun --account=venkvis_gpu --partition=gpu --gres=gpu:1 --mem=2G --time=30 --pty bash 