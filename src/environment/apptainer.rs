use super::{which, EnvironmentManager};
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

pub struct Apptainer {
    path: PathBuf,
    definition: PathBuf,
}

impl Apptainer {
    fn get_image(&self) -> PathBuf {
        let mut image = PathBuf::from(&self.definition);
        image.set_extension(".sif");
        return image;
    }
}

impl EnvironmentManager for Apptainer {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn install(&self) -> Result<Output, io::Error> {
        let definition = &self.definition;
        let image = self.get_image();
        if let Some(path) = apptainer() {
            return Command::new(path)
                .arg("build")
                .arg(image)
                .arg(definition)
                .output();
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Apptainer/singularity not installed",
            ));
        }
    }

    fn with_env(&self, cmd: Command) -> Command {
        let apptainer = apptainer().expect("apptainer/singularity to be installed");
        let mut cmd_env = Command::new(apptainer);
        if let Some(cwd) = cmd.get_current_dir() {
            cmd_env.current_dir(cwd);
        }
        cmd_env
            .arg("run")
            .arg("--no-home")
            .arg("--nv")
            .arg(self.get_image())
            .arg(cmd.get_program())
            .args(cmd.get_args());
        return cmd;
    }
}

fn apptainer() -> Option<PathBuf> {
    if let Some(path) = which("apptainer") {
        return Some(path);
    }
    if let Some(path) = which("singularity") {
        return Some(path);
    }
    None
}
