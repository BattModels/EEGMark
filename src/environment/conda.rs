use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

use super::env_minimal;
use super::with_envs;
use super::EnvironmentManager;

#[derive(Debug)]
pub struct Conda {
    path: PathBuf,
}

impl Conda {
    pub fn new(dir: &PathBuf) -> Result<Self, ()> {
        let env = Conda {
            path: PathBuf::from(dir),
        };
        if dir.join("environment.yml").is_file() {
            return Ok(env);
        }
        Err(())
    }
}

impl EnvironmentManager for Conda {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> io::Result<ExitStatus> {
        if self.path.join(".conda").exists() {
            fs::remove_dir_all(&self.path.join(".conda"))
                .expect("should be possible to delete an existing .conda directory");
        }
        env_minimal(Command::new("conda"))
            .current_dir(self.path())
            .arg("env")
            .arg("create")
            .arg("--prefix")
            .arg(".conda")
            .arg("--file")
            .arg("environment.yml")
            .spawn()?
            .wait()
    }

    fn with_env(&self, cmd: Command) -> Command {
        let mut conda_cmd = env_minimal(Command::new("conda"));
        conda_cmd
            .current_dir(self.path())
            .arg("run")
            .arg("--prefix")
            .arg(".conda")
            .arg("--live-stream")
            .arg(cmd.get_program())
            .args(cmd.get_args());
        with_envs(&mut conda_cmd, &cmd);
        return conda_cmd;
    }
}
