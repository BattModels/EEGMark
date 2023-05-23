import subprocess
import logging
from pathlib import Path
import spack
from spack.environment.shell import activate as spack_activate


SCORE_LINE = re.compile(r"^score: ?(?P<score>[\d,+-]+)$")


class Benchmark:
    def __init__(self, name, pkgman: PackageManger, runner: Runner):
        self.name = name
        self.pkgman = pkgman
        self.script = runner = Runner

    def __str__(self):
        return self.name

    def run(self):
        """Run a single run of the benchmark, returning a score"""
        logging.info("running benchmarks for %s", self)

        # Run script, recording the run time
        start = time.perf_counter_ns()
        output = self.pkgman.execute_command(f"bash {self.script.resolve()}")
        elapsed = time.perf_counter_ns() - start

        # Score the benchmark
        lastline = output.splitlines()[-1]
        m = SCORE_LINE.match(lastline)
        score = m["score"] if m else elapsed

        logging.info("ran benchmark for %s in %.6e seconds", self, elapsed * 1e-9)
        return score

    def install(self):
        """Install dependencies for the benchmark"""
        logging.info("installing dependency for %s", self)
        start = time.perf_counter()
        self.pkgman.setup()
        self.pkgman.concretize()
        self.pkgman.install()
        elapsed = time.perf_counter() - start
        logging.info("finished installing dependencies for %s in %.3s", self, elapsed)

    @staticmethod
    def from_folder(path):
        # Locate package manager
        pkgman = None
        for p in [SpackPipManger, SpackManager, CondaManager]:
            try:
                pkgman = p(path)
            except:
                continue
        logging.info("selected package manger %s for %s", pkgman, path)

        # Locate script
        script = Path(path, "run.sh")
        assert script.isfile()
        return Benchmark(path, pkgman, script)


class PackageManger:
    """Install dependencies for a benchmark"""

    @abstractmethod
    def setup(self):
        """Ensure the package manager is ready"""

    def concretize(self):
        """Select what to install"""
        pass

    @abstractmethod
    def install(self):
        """Install all dependencies for the benchmark"""

    @abstractmethod
    def execute_command(self, cmd):
        """Run a shell command in the installed environment"""


def SpackManager(PackageManger):
    def __init__(self, path):
        assert Path(path, "spack.yaml").isfile()
        self.environment = spack.environment.Environment(path)

    def setup(self):
        # Spack is a dependency of EEGMark
        pass

    def concretize(self):
        self.environment.concretize(force=True)
        self.environment.write()

    def install(self):
        self.environment.install_all()
        self.environment.write()

    def execute_command(self, cmd):
        """Run a shell command in the installed environment"""
        exe = Executable(cmd)
        exe.add_default_envmod(spack_activate(self.environment))
        return exe()


def SpackPipManger(SpackManager):
    def __init__(self, path):
        # Look for a pip requirement's file
        self.pip_requirements = Path(path, "requirements.txt").resolve()
        assert self.pip_requirements.isfile()

        # Look for a spack environment file
        super().__init__(self, path)

        # Check that pip will be installed
        has_pip = self.environment.matching_spec(spack.spec.Spec("py-pip")) is not None
        if not has_pip:
            logging.warn(
                "found spack.yaml and requirements.txt for %s, but 'py-pip' was not installed in the environment. You may want to fix that"
            )
            assert has_pip

    def install(self):
        logging.debug("installing spack dependencies with %s", self.environment.name)
        super().install()

        # Install requirements with pip
        logging.debug("installing pip dependencies for %s", self.pip_requirements)
        self.execute_command(f"python -m pip install -f {self.pip_requirements}")


def spack_install(spec):
    """Replicate `spack install spec`"""
    spec = spack.cmd.parse_specs("miniconda3", concretize=True)[0]
    if not spec.installed:
        installer = spack.installer.PackageInstall([(spec, {})])
        installer.install()

    return spec.prefix


def CondaManager(PackageManger):
    def __init__(self, path):
        self.environment = Path(path, "environment.yml").resolve()
        assert self.environment.isfile()

    def setup(self):
        self.conda = Path(spack_install("miniconda3"), "bin", "conda")

    def concretize(self):
        pass

    def install(self):
        subprocess.run(
            [
                self.conda,
                "create",
                "-f",
                str(self.environment),
                "--prefix",
                Path(self.environment.parent, ".conda"),
            ]
        )

    def execute_command(self, cmd):
        cwd(self.environment.parent)
        out = subprocess.run(
            [self.conda, "run", "--prefix", Path(cwd, ".conda"), "--cwd", cwd, cmd],
            capture_output=True,
        )
        return out.stdout.decode("UTF-8")
