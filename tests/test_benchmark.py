from pathlib import Path
from EEGMark.benchmark import Benchmark


class BenchmarkDemos:
    def __init__(path):
        self.path = path
        assert Path(path).isdir()

    def test_install(self):
        b = Benchmark(self.path)
        b.install()
        score = b.run()
        assert score >= 0
        return score


class TestSpackPip(BenchmarkDemos):
    def __init__():
        super().__init__(self, Path(Path(__file__).parent, "spack_pip_demo"))

    def test_install(self):
        score = super.test_install()
        assert score == 2023.05
