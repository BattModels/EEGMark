use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

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
        if env.environment().is_file() {
            return Ok(env);
        }
        Err(())
    }
    fn virtual_environment(&self) -> PathBuf {
        self.path().join(".conda")
    }

    fn environment(&self) -> PathBuf {
        self.path().join("environment.yml")
    }
}

impl EnvironmentManager for Conda {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> Result<Output, io::Error> {
        let venv = self.virtual_environment();
        let env = self.environment();
        if venv.exists() {
            fs::remove_dir_all(&venv)
                .expect("should be possible to delete an existing .conda directory");
        }
        Command::new("conda")
            .current_dir(self.path())
            .arg("env")
            .arg("create")
            .arg("--prefix")
            .arg(venv)
            .arg("--file")
            .arg(env)
            .output()
    }

    fn command(&self, program: &str) -> Command {
        let mut cmd = Command::new("conda");
        cmd.current_dir(self.path())
            .arg("run")
            .arg("--prefix")
            .arg(self.virtual_environment())
            .arg(program);
        return cmd;
    }
}
