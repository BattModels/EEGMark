use super::environment::Environment;
use std::path::PathBuf;

struct Benchmark {
    env: Environment,
    run_script: PathBuf,
}
