use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::str;

use super::{has_yaml, which, with_envs, EnvironmentManager};

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
        let spack = which("spack").expect("Expected spack to be on the path");
        let activate_script = Command::new(spack)
            .env_clear()
            .env("PATH", "/usr/local/bin:/usr/bin:/bin")
            .current_dir(self.path())
            .arg("env")
            .arg("activate")
            .arg("--sh")
            .arg("--dir")
            .arg(".")
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
        let mut spack_root = which("spack").unwrap();
        spack_root.pop();
        spack_root.pop();
        env.insert(
            String::from("SPACK_ROOT"),
            String::from(spack_root.to_str().unwrap()),
        );

        return env;
    }
}
impl EnvironmentManager for Spack {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> io::Result<ExitStatus> {
        Command::new("spack")
            .arg("-e")
            .arg(self.path())
            .arg("install")
            .spawn()
            .unwrap()
            .wait()
    }

    fn with_env(&self, cmd: Command) -> Command {
        let mut spack_cmd = Command::new(cmd.get_program());
        spack_cmd
            .current_dir(self.path())
            .env_clear()
            .envs(self.spack_env())
            .args(cmd.get_args());
        with_envs(&mut spack_cmd, &cmd);
        return spack_cmd;
    }
}
