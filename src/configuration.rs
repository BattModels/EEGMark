use crate::environment::which;
use dirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::{fs, str};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
enum StandardTools {
    Spack,
    Conda,
    Juliaup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    path: Vec<PathBuf>,
}

impl Config {
    fn config_directory() -> PathBuf {
        dirs::home_dir().unwrap().join(".config/eegmark")
    }
    pub fn from_file() -> io::Result<Config> {
        match fs::read_to_string(Config::config_directory().join("config")) {
            Ok(s) => match serde_yaml::from_str(&s) {
                Ok(config) => Ok(config),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "failed to parse file",
                )),
            },
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    let config = setup_environment();
                    config.to_disk().unwrap();
                    Ok(config)
                } else {
                    Err(e)
                }
            }
        }
    }
    pub fn to_disk(self: &Self) -> io::Result<()> {
        let s = serde_yaml::to_string(self).expect("should Serialize");
        fs::create_dir_all(Config::config_directory()).expect("failed to create config directory");
        fs::write(Config::config_directory().join("config"), s)
    }
    pub fn get_path_env(self: &Self) -> String {
        let paths = self.path.to_owned();
        let paths: Vec<String> = paths
            .into_iter()
            .map(|f| String::from(f.to_str().unwrap()))
            .collect();
        paths.join(":")
    }
}

/// Ensures that the minimal set of tools has been installed on the system
pub fn setup_environment() -> Config {
    // Create Vector of Paths
    let mut paths = Vec::new();
    paths.push(PathBuf::from("/usr/local/bin/"));
    paths.push(PathBuf::from("/usr/bin"));
    paths.push(PathBuf::from("/bin"));

    // Add paths to StandardTools
    let tools = StandardTools::iter().collect::<Vec<_>>();
    for tool in tools {
        let p = match tool {
            StandardTools::Spack => try_which("spack", ensure_spack),
            StandardTools::Conda => try_which("conda", ensure_conda),
            StandardTools::Juliaup => try_which("juliaup", ensure_juliaup),
        };
        match p {
            Ok(p) => paths.push(p),
            Err(msg) => panic!("{}", msg),
        }
    }
    Config { path: paths }
}

fn try_which(cmd: &str, fallback: fn() -> Result<PathBuf, String>) -> Result<PathBuf, String> {
    if let Some(p) = which(cmd) {
        return Ok(p.parent().unwrap().to_path_buf());
    } else {
        return fallback();
    }
}

fn ensure_spack() -> Result<PathBuf, String> {
    let out = Command::new("git")
        .arg("clone")
        .arg("--filter=tree:0")
        .arg("https://github.com/spack/spack.git")
        .arg("spack")
        .spawn()
        .unwrap()
        .wait();
    match out {
        Ok(_) => return Ok(fs::canonicalize("spack").unwrap()),
        Err(e) => return Err(e.to_string()),
    }
}

fn ensure_conda() -> Result<PathBuf, String> {
    if let Ok(spack_dir) = try_which("spack", ensure_spack) {
        return ensure_spack_package(spack_dir.join("spack").to_str().unwrap(), "miniconda3");
    } else {
        return Err(String::from("unable to locate/install spack"));
    }
}

fn ensure_juliaup() -> Result<PathBuf, String> {
    // Look in default installation directory
    let juliaup_path = if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".juliaup/bin/juliaup")
    } else {
        return Err(String::from("unable to expand $HOME"));
    };
    if let Ok(path) = fs::canonicalize(juliaup_path) {
        return Ok(path.parent().unwrap().to_path_buf());
    }

    // Install juliaup
    let install_script = Command::new("curl")
        .arg("-fsSL")
        .arg("https://install.julialang.org")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start download of install script");
    let out = Command::new("sh")
        .stdin(Stdio::from(install_script.stdout.unwrap()))
        .arg("-s")
        .arg("--")
        .arg("-y")
        .spawn()
        .expect("failed to start install script")
        .wait()
        .expect("juliaup install failed");

    // Look in the default location again
    if let Ok(path) = fs::canonicalize("~/.juliaup/bin/juliaup") {
        return Ok(path.parent().unwrap().to_path_buf());
    }
    Err(String::from("Failed to install juliaup"))
}

fn ensure_spack_package(spack: &str, spec: &str) -> Result<PathBuf, String> {
    let out = Command::new(spack)
        .arg("location")
        .arg("--install-dir")
        .arg(spec)
        .output()
        .unwrap();
    if out.status.success() {
        // Found the install directory
        let p = str::from_utf8(out.stdout.as_slice()).unwrap();
        return Ok(PathBuf::from(p.trim()));
    }

    // If multiple packages match -> Let the user pick one
    let err_msg = String::from_utf8(out.stderr).unwrap();
    if !Regex::new("no installed packages")
        .unwrap()
        .is_match(&err_msg)
    {
        return Err(err_msg);
    }

    // Install the package
    match Command::new(spack)
        .arg("install")
        .arg(spec)
        .spawn()
        .unwrap()
        .wait()
    {
        Ok(_) => return ensure_spack_package(spack, spec),
        Err(_) => return Err(format!("failed to install {} using spack", spec)),
    }
}
