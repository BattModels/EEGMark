use clap::{arg, command, value_parser, ArgMatches, Command};
use eegmark::environment::Environment;
use std::{fs, path::PathBuf};

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("install", sub_matches)) => install_cmd(sub_matches),
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
                .arg(
                    arg!([dir] "Environment directory/directories to install")
                        .default_value("./benchmarks")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
}

fn install_cmd(m: &ArgMatches) {
    let dir = m
        .get_one::<PathBuf>("dir")
        .expect("Missing directory to install");
    if !dir.is_dir() {
        panic!("Expected {:?} to exist and be a directory", dir);
    }
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if let Ok(env) = Environment::from_folder(&path) {
            print!("Found environement for {:?}", path);
            match env.install() {
                Ok(out) => println!("{:?}", out),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
