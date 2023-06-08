import os
import json
import yaml

out = {}
nextline = False
logfile = 'log.lammps'
path = ''
if not os.path.isfile(logfile):
    path = 'benchmarks/lammps-allegro'
f = open(os.path.join(path,logfile))
for line in f.readlines():
    if 'Performance' in line:
        print(line)
        out['raw1'] = line
        _, p1, u1, p2, u2 = line.split(' ')
        out['simulation length'] = float(p1)
        out['simulation length units'] = u1[:-1]
        out['timesteps/s'] = float(p2)
        nextline = True
    elif nextline:
        print(line)
        out['raw2'] = line
        out['utilization'] = line.split(' ')[0]
        nextline = False

f.close()

with open(
    os.path.join(path,'out.yaml'),'w'
) as fl:
    yaml.dump(out,fl)