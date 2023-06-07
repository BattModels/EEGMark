#!/bin/bash
set -e
spack load apptainer || spack install apptainer && spack load apptainer
if [ ! -f lolcow_latest.sif ]; then
    apptainer pull docker://godlovedc/lolcow
fi
