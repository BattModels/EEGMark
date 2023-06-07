#!/bin/bash
# How fast can we get 10 insightful quotes from a cow?
eval $(spack load --sh apptainer)
for i in {1..10}; do
    apptainer run docker://godlovedc/lolcow
done
