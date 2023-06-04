use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

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

    fn install(&self) -> io::Result<ExitStatus> {
        Command::new("julia")
            .current_dir(self.path())
            .arg("--startup-file=no")
            .arg("--project")
            .arg("--eval")
            .arg("using Pkg; Pkg.instantiate()")
            .spawn()
            .unwrap()
            .wait()
    }

    fn with_env(&self, cmd: Command) -> Command {
        let mut cmd = Command::from(cmd);
        cmd.env("JULIA_PROJECT", self.path())
            .env("JULIA_LOAD_PATH", "@")
            .current_dir(self.path());
        return cmd;
    }
}
