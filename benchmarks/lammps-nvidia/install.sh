chmod +x run.sh

mkdir logs

conda create -n env python=3.9
conda activate env
conda install pyyaml