use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str;

// Modules for environment types
mod apptainer;
mod conda;
mod generic;
mod julia;
mod spack;
use apptainer::Apptainer;
use conda::Conda;
use generic::Generic;
use julia::Julia;
use spack::Spack;

pub struct Environment {
    env: Box<dyn EnvironmentManager>,
}

impl Environment {
    pub fn path(&self) -> &PathBuf {
        self.env.path()
    }

    pub fn install(&self) -> Result<Output, io::Error> {
        self.env.install()
    }

    pub fn with_env(&self, cmd: Command) -> Command {
        self.env.with_env(cmd)
    }

    pub fn from_folder(dir: &PathBuf) -> Result<Self, ()> {
        if let Ok(env) = Generic::new(dir) {
            return Ok(Environment { env: Box::new(env) });
        } else if let Ok(env) = Spack::new(dir) {
            return Ok(Environment { env: Box::new(env) });
        } else if let Ok(env) = Julia::new(dir) {
            return Ok(Environment { env: Box::new(env) });
        } else if let Ok(env) = Conda::new(dir) {
            return Ok(Environment { env: Box::new(env) });
        }
        Err(())
    }
}

pub trait EnvironmentManager {
    fn path(&self) -> &PathBuf;

    fn install(&self) -> Result<Output, io::Error>;

    fn with_env(&self, cmd: Command) -> Command;
}

fn has_yaml(dir: &PathBuf, name: &str) -> Result<PathBuf, ()> {
    let mut definition = dir.join(name);
    for ext in ["yaml", "yml"] {
        definition.set_extension(ext);
        if definition.is_file() {
            return Ok(PathBuf::from(definition));
        }
    }
    return Err(());
}

// Add/Update/Remove environment variables in `cmd`
// based on `ref_cmd.get_envs()`
fn with_envs(cmd: &mut Command, ref_cmd: &Command) {
    for (key, value) in ref_cmd.get_envs() {
        if let Some(v) = value {
            cmd.env(key, v);
        } else {
            cmd.env_remove(key);
        }
    }
}

// Locate the path to an executable using which
fn which(cmd: &str) -> Option<PathBuf> {
    let which = Command::new("which")
        .arg(cmd)
        .output()
        .expect("Expected which to be on the PATH");
    match str::from_utf8(which.stdout.as_slice()) {
        Ok(path) => return Some(PathBuf::from(path.trim())),
        Err(_) => return None,
    }
}
