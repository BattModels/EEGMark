import re
import math


def find_parameters(fid, keys):
    re_param = re.compile(r"^(.*?)=(.*)")
    data = dict()
    for line in fid.readlines():
        m = re_param.match(line)
        if m:
            name = m.group(1)
            if name in keys:
                data[name] = float(m.group(2))

    return data


def geomean(*args):
    return math.pow(math.prod(args), 1 / len(args))


with open("./hpccoutf.txt", "r") as fid:
    data = find_parameters(
        fid,
        [
            "MPIFFT_Gflops",
            "SingleFFT_Gflops",
            "StarFFT_Gflops",
            "MPIRandomAccess_GUPs",
            "StarRandomAccess_GUPs",
            "SingleRandomAccess_GUPs",
            "RandomlyOrderedRingBandwidth_GBytes",
            "SingleSTREAM_Triad",
            "StarSTREAM_Triad",
            "StarDGEMM_Gflops",
            "SingleDGEMM_Gflops",
            "HPL_Tflops",
        ],
    )
    scores = {
        "hpl": geomean(
            data["HPL_Tflops"] * 1000,
            data["SingleDGEMM_Gflops"],
            data["StarDGEMM_Gflops"],
        ),
        "random": geomean(
            data["MPIRandomAccess_GUPs"],
            data["StarRandomAccess_GUPs"],
            data["SingleRandomAccess_GUPs"],
        ),
        "stream": geomean(
            data["SingleSTREAM_Triad"],
            data["StarSTREAM_Triad"],
        ),
        "fft": geomean(
            data["MPIFFT_Gflops"],
            data["SingleFFT_Gflops"],
            data["StarFFT_Gflops"],
        ),
    }
    print(scores)
    print("score: %f" % geomean(*scores.values()))
