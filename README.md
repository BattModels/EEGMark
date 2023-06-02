# EEGMark
A benchmarking suite for the [Electrochemical Energy Group](https://www.cmu.edu/me/venkatgroup/), similar to existing benchmarking suites (https://openbenchmarking.org/suites), but geared toward HPC.

## Running the Benchmarks
Tooling development is ongoing, see [#oxidize](), but generally is split into two steps:

1. Installation: Preparing an environment in which to run the benchmark
2. Running: Actually running the benchmarks


## Adding New Benchmarks

Open a [pull request](https://github.com/BattModels/EEGMark/pulls) to add a new benchmark. Benchmarks should follow the following structure:

```
- benchmarks/
    + new-awesome-benchmark/
        - install.sh    # Script to setup the benchmarking environment (Automated for common environments)
        - run.sh        # This is the benchmarking script
        - config.yml    # Optional. Additonal information about the benchmark (i.e. does it need a GPU)
```

Benchmarks should:
- Report a relevant, commonly used metric (i.e. ns/day) for scientifically relevant systems
    - Quality of Life benchmarks (i.e. precompile times for julia) are acceptable
- Make efficient use of the hardware (Aim to have 100% utilization or be single threaded)
- Should run in less than 1 hour and at most 4 hours (TBD)

> Remember: Ultimately we're interested in Performance per Dollar. RAM is expensive, Disk Storage is not.
> when possible, benchmarks should reflect this (i.e. use `disk_io` in quantum-espresso)

### Installing the Environment

We (will) support automated installs for the following environments:

| Manager | Required Files  | Notes |
|---------|-----------------|-------|
| Spack   | `spack.yaml`      |       |
| Conda   | `environment.yml` | Hand crafted is better, `conda env export` sometime gets overspeced |
| Julia   | `Project.toml`    |  |
| Generic | `install.sh`      |  |

> Apptainer Support: Desired, not sure on the best interface. For now, use spack and plan on downloading a built
> image from somewhere (public registry, google drive, etc.). If this applies to you ping @awadell1.

### The Run Script: `run.sh`

Once the environment, has been installed this script is used to define the actual benchmark. Roughly it is called in the following manner:

```shell
bash install.sh
activate environment # i.e. `conda activate` or `spack env activate -d .`
bash run.sh
```

The score for the benchmark by default is the inverse of wall time (in days) for `run.sh`. Alternatively, `run.sh` can print a
score to stdout as it's final line of output. The score must meet the following requirements:

- It must be a non-negative, finite number
- Larger scores are, by default, better, see [Configuration](#the-config-file)
- It must be printed as: `score: %f` (i.e. scientific notation is not allowed)
    - python: `printf(f"score: {score:f}")`
    - julia: `using Printf; @printf("score: %f", score)`
    - bash: `echo $SCORE`
- It must be the last line of output from `run.sh`
- The wall time  (in seconds) for the `run.sh`

### The Configuration File

The configuration file is still being worked out, the current format is shown below:

```yaml
benchmark:
    maximize_score: True    # False, if smaller scores are better
hardware:
    needs_gpu: False        # True, if a gpu is required
```

If no `config.yml` file is provided the default is used.
