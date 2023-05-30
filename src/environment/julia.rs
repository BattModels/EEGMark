use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

use super::EnvironmentManager;

#[derive(Debug)]
pub struct Julia {
    path: PathBuf,
}

impl Julia {
    pub fn new(dir: &PathBuf) -> Result<Self, ()> {
        if dir.join("Project.toml").is_file() {
            return Ok(Julia {
                path: PathBuf::from(dir),
            });
        }
        Err(())
    }
}

impl EnvironmentManager for Julia {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> Result<Output, io::Error> {
        Command::new("julia")
            .current_dir(self.path())
            .arg("--startup-file=no")
            .arg("--project")
            .arg("--eval")
            .arg("using Pkg; Pkg.instantiate()")
            .output()
    }

    fn command(&self, program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.env("JULIA_PROJECT", self.path())
            .env("JULIA_LOAD_PATH", "@")
            .current_dir(self.path());
        return cmd;
    }
}
