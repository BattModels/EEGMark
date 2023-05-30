use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str;

// Modules for environment types
mod conda;
mod generic;
mod julia;
mod spack;
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

    pub fn command(&self, program: &str) -> Command {
        self.env.command(program)
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

    fn command(&self, program: &str) -> Command;
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
