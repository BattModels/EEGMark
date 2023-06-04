use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

use super::EnvironmentManager;

#[derive(Debug)]
pub struct Generic {
    path: PathBuf,
}

impl Generic {
    pub fn new(dir: &PathBuf) -> Result<Self, ()> {
        if dir.join("install.sh").is_file() {
            return Ok(Generic {
                path: PathBuf::from(dir),
            });
        }
        Err(())
    }
}

impl EnvironmentManager for Generic {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> io::Result<ExitStatus> {
        Command::new("bash")
            .current_dir(self.path())
            .arg(self.path().join("install.sh"))
            .spawn()
            .unwrap()
            .wait()
    }

    fn with_env(&self, cmd: Command) -> Command {
        let mut cmd = Command::from(cmd);
        cmd.current_dir(self.path()).env_clear();
        return cmd;
    }
}
