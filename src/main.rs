use clap::{arg, command, value_parser, ArgMatches, Command};
use eegmark::benchmark::Benchmark;
use eegmark::environment::Environment;
use std::path::PathBuf;
use std::process;

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", sub_matches)) => install_cmd(sub_matches),
        Some(("run", sub_matches)) => run_cmd(sub_matches),
        Some(("shell", sub_matches)) => run_shell(sub_matches),
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
                .about("Install a benchmarking environment")
                .arg(
                    arg!([dir] "Environment directory to install")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("run").about("Run benchmark").arg(
                arg!([dir] "Environment directory to run").value_parser(value_parser!(PathBuf)),
            ),
        )
        .subcommand(
            Command::new("shell")
                .about("Launch a shell within the environment")
                .arg(arg!([dir] "Environment directory").value_parser(value_parser!(PathBuf))),
        )
}

fn install_cmd(m: &ArgMatches) {
    let dir = m
        .get_one::<PathBuf>("dir")
        .expect("Missing directory to install");
    let output = Benchmark::from_folder(dir)
        .expect("Not a benchmark directory")
        .install()
        .unwrap();
    println!("{:?}", output);
}

fn run_cmd(m: &ArgMatches) {
    let dir = m.get_one::<PathBuf>("dir").expect("Missing directory");
    let bench = Benchmark::from_folder(dir).expect("Not a benchmark directory");
    let trial = bench.run().unwrap();
    println!("Result: {} => {:?}", bench.name().to_str().unwrap(), trial);
}

fn run_shell(m: &ArgMatches) {
    let dir = m.get_one::<PathBuf>("dir").expect("Missing directory");
    let env = Environment::from_folder(dir).expect("Not a benchmark directory");
    let mut shell = env.with_env(process::Command::new("/bin/bash"));
    shell.spawn().unwrap().wait().unwrap();
}
