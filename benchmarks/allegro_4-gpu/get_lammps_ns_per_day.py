with open('log.lammps', 'r') as f:
    lines = f.readlines()

for line in lines:
    if line.startswith('Performance'):
        ns_per_day=float(line.split(' ')[1])
        break

print(f'score: {ns_per_day}')
    