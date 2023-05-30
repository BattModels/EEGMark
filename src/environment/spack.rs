use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str;

use super::{has_yaml, EnvironmentManager};

#[derive(Debug)]
pub struct Spack {
    path: PathBuf,
}
impl Spack {
    pub fn new(dir: &PathBuf) -> Result<Self, ()> {
        if let Ok(_) = has_yaml(dir, "spack") {
            return Ok(Spack {
                path: PathBuf::from(dir),
            });
        } else {
            return Err(());
        }
    }
    fn spack_env(&self) -> HashMap<String, String> {
        // Locate Spack
        let which_spack = Command::new("which")
            .arg("spack")
            .output()
            .expect("Expected which to be on the PATH");
        let spack_path = str::from_utf8(which_spack.stdout.as_slice())
            .expect("Expected spack to be on the PATH");
        let spack_path = PathBuf::from(spack_path);
        let spack_path = spack_path
            .parent()
            .expect("expected spack-exe to be in a directory");

        let activate_script = Command::new(spack_path.to_str().unwrap())
            .current_dir(self.path())
            .env_clear()
            .arg("env")
            .arg("activate")
            .arg("--sh")
            .arg("--dir")
            .arg(self.path())
            .output()
            .expect("expected command to be correct");
        let activate_script = str::from_utf8(activate_script.stdout.as_slice())
            .expect("Expected output to be non-empty");

        let re = Regex::new(r"export (\S+)=(\S*);").unwrap();
        let mut env = HashMap::new();
        for line in activate_script.lines() {
            if let Some(m) = re.captures(line) {
                env.insert(String::from(&m[1]), String::from(&m[2]));
            }
        }
        return env;
    }
}
impl EnvironmentManager for Spack {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> Result<Output, io::Error> {
        Command::new("spack")
            .arg("-e {self.path()}")
            .arg("install")
            .output()
    }

    fn command(&self, program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.current_dir(self.path())
            .env_clear()
            .envs(self.spack_env());
        return cmd;
    }
}
