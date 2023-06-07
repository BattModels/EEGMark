use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
use eegmark::benchmark::Benchmark;
use eegmark::configuration;
use eegmark::environment::Environment;
use std::path::PathBuf;
use std::process;
use walkdir;

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", sub_matches)) => install_cmd(sub_matches),
        Some(("run", sub_matches)) => run_cmd(sub_matches),
        Some(("shell", sub_matches)) => run_shell(sub_matches),
        Some(("init", sub_matches)) => run_init(sub_matches),
        Some(("yolo", sub_matches)) => yolo(sub_matches),
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install benchmarking environments")
                .arg(Arg::new("dry-run").short('n').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("dir")
                        .help("root path to start searching for benchmarks")
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run benchmark")
                .arg(Arg::new("dry-run").short('n').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("dir")
                        .help("root path to start searching for benchmarks")
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("yolo")
                .about("Run and install benchmark")
                .arg(Arg::new("dry-run").short('n').action(ArgAction::SetTrue))
                .arg(
                    Arg::new("dir")
                        .help("root path to start searching for benchmarks")
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("shell")
                .about("Launch a shell within the environment")
                .arg(arg!([dir] "Environment directory").value_parser(value_parser!(PathBuf))),
        )
        .subcommand(Command::new("init").about("Setup the machine for benchmarking"))
}

// Source: https://docs.rs/walkdir/latest/walkdir/struct.IntoIter.html
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn install_cmd(m: &ArgMatches) {
    let dry_run = m.get_flag("dry-run");
    let root_directory = m.get_one::<PathBuf>("dir").expect("DIR is required");
    let mut walker = walkdir::WalkDir::new(root_directory).into_iter();
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if !entry.file_type().is_dir() || is_hidden(&entry) {
            continue;
        }
        if let Ok(env) = Environment::from_folder(&entry.into_path()) {
            if dry_run {
                println!("would install {:?}", env.path());
            } else if env.install().is_err() {
                println!("ERROR: unable to install {:?}", env.path());
            }

            // Found environment, stop descending
            walker.skip_current_dir();
        }
    }
}
fn run_cmd(m: &ArgMatches) {
    let dry_run = m.get_flag("dry-run");
    let root_directory = m.get_one::<PathBuf>("dir").expect("DIR is required");
    let mut walker = walkdir::WalkDir::new(root_directory).into_iter();
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if !entry.file_type().is_dir() || is_hidden(&entry) {
            continue;
        }
        if let Ok(bench) = Benchmark::from_folder(&entry.into_path()) {
            // Found environment, stop descending
            walker.skip_current_dir();

            if dry_run {
                println!("would run {:?}", bench.name());
                continue;
            }
            if let Some(trial) = bench.run() {
                println!(
                    "Finished benchmarking {} in {}s.",
                    bench.name(),
                    trial.elapsed()
                );
                if let Err(e) = trial.to_disk(&bench) {
                    println!(
                        "ERROR: failed to save benchmarking results for {}: {:?}.\n{}",
                        bench.name(),
                        trial,
                        e,
                    );
                }
            } else {
                println!("ERROR: benchmarking failed for {}", bench.name());
            }
        }
    }
}

fn run_shell(m: &ArgMatches) {
    let dir = m.get_one::<PathBuf>("dir").expect("Missing directory");
    let env = Environment::from_folder(dir).expect("Not a benchmark directory");
    let mut shell = env.with_env(process::Command::new("/bin/bash"));
    shell.spawn().unwrap().wait().unwrap();
}

fn run_init(_m: &ArgMatches) {
    let config = configuration::setup_environment();
    println!("{:?}", config);
    config
        .to_disk()
        .expect("Failed to setup machine for benchmarking, see above for info");
}

fn yolo(m: &ArgMatches) {
    let dry_run = m.get_flag("dry-run");
    let root_directory = m.get_one::<PathBuf>("dir").expect("DIR is required");
    let mut walker = walkdir::WalkDir::new(root_directory).into_iter();
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if !entry.file_type().is_dir() || is_hidden(&entry) {
            continue;
        }
        if let Ok(bench) = Benchmark::from_folder(&entry.into_path()) {
            // Found environment, stop descending
            walker.skip_current_dir();

            if dry_run {
                println!("would install and run {:?}", bench.name());
                continue;
            }
            if let Err(e) = bench.install() {
                println!("ERROR: failed to install {}. err: {}", bench.name(), e);
                continue;
            }
            if let Some(trial) = bench.run() {
                println!(
                    "Finished benchmarking {} in {}s.",
                    bench.name(),
                    trial.elapsed()
                );
                if let Err(e) = trial.to_disk(&bench) {
                    println!(
                        "ERROR: failed to save benchmarking results for {}: {:?}. err: {}",
                        bench.name(),
                        trial,
                        e,
                    );
                }
            } else {
                println!("ERROR: benchmarking failed for {}", bench.name());
            }
        }
    }
}
