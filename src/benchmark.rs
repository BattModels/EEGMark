use crate::configuration::Config;
use crate::environment::Environment;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{ChildStdout, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};
use std::{cmp, fs, io, thread};
use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};
use uuid::Uuid;

pub struct Benchmark {
    environment: Environment,
    script: PathBuf,
}

impl Benchmark {
    /// Construct a benchmark from a folder
    pub fn from_folder(dir: &PathBuf) -> Result<Self, ()> {
        let script = dir.join("run.sh");
        let has_parent = match script.parent() {
            Some(d) => d.exists(),
            None => false,
        };
        if !script.is_file() && has_parent {
            return Err(());
        }
        match Environment::from_folder(dir) {
            Ok(environment) => {
                return Ok(Benchmark {
                    environment,
                    script,
                })
            }
            Err(_) => return Err(()),
        };
    }
    /// Return the name of the benchmark
    pub fn name(&self) -> &str {
        self.script
            .parent()
            .expect("run scripts should be within a folder")
            .file_name()
            .expect("directories have filename")
            .to_str()
            .unwrap()
    }

    pub fn install(&self) -> io::Result<ExitStatus> {
        return self.environment.install();
    }

    fn command(&self) -> Command {
        let mut cmd = Command::new("/bin/bash");
        cmd.arg(self.script.file_name().unwrap())
            .current_dir(self.script.parent().unwrap());
        self.environment.with_env(cmd)
    }

    pub fn run(&self) -> Option<Trial> {
        run_benchmark(self.command())
    }

    pub fn results_directory(&self) -> PathBuf {
        Config::config_directory().join("results").join(self.name())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trial {
    score: f64,
    walltime: Duration,
    max_memory: u64,
    cpu_usage: f32,
}

impl Trial {
    pub fn elapsed(self: &Self) -> u64 {
        return self.walltime.as_secs();
    }
    pub fn to_disk(self: &Self, benchmark: &Benchmark) -> io::Result<()> {
        let id = Uuid::new_v4().hyphenated().to_string();
        let trial_path = benchmark.results_directory().join(id);
        fs::create_dir_all(&trial_path)?;

        let buff = serde_json::to_string(&self)?;
        let mut fid = File::create(trial_path.join("trial.json"))?;
        match fid.write(buff.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn run_benchmark(cmd: Command) -> Option<Trial> {
    let mut cmd = Command::from(cmd);
    cmd.stdin(Stdio::null()).stdout(Stdio::piped());

    let polling_rate = Duration::from_millis(500);

    // Launch benchmark and track relevant stats
    let mut max_memory: u64 = 0;
    let mut sys = System::new();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    let starttime = Instant::now();
    let mut process = match cmd.spawn() {
        Ok(p) => p,
        Err(e) => {
            println!("dang {:?}", e);
            return None;
        }
    };
    while process.try_wait().unwrap().is_none() {
        sys.refresh_memory();
        max_memory = cmp::max(sys.total_memory(), max_memory);
        thread::sleep(polling_rate);
    }

    // Record System Information
    let walltime = starttime.elapsed();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    let mut cpu_usage: f32 = 0.0;
    for cpu in sys.cpus() {
        cpu_usage += cpu.cpu_usage();
    }

    // Check return code
    let status = process.try_wait().expect("should have exited").unwrap();
    if !status.success() {
        return None;
    }

    // Look for a score
    let score = score_benchmark(process.stdout.unwrap(), &walltime);

    Some(Trial {
        score,
        walltime,
        max_memory,
        cpu_usage,
    })
}

fn score_benchmark(mut stdout: ChildStdout, elapsed: &Duration) -> f64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^score: ?([\d\.,+-]+)$").unwrap();
    }
    let mut buffer = String::new();
    stdout
        .read_to_string(&mut buffer)
        .expect("reading stdout from process");
    let last_line = buffer.lines().last().unwrap_or("");
    if let Some(cap) = RE.captures(last_line) {
        return cap[1].parse::<f64>().unwrap();
    }

    // Fall Back to Daily Rate
    return (24.0 * 60.0 * 60.0) / elapsed.as_secs_f64();
}
