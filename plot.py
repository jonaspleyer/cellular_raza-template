import matplotlib.pyplot as plt
import numpy as np
import json
from glob import glob
from pathlib import Path
from typing import Optional
import pandas as pd


def get_last_output_dir(output_dir: Optional[Path] = None):
    if output_dir == None:
        return Path(sorted(glob("out/*"))[-1])
    else:
        return output_dir


def get_all_iterations(output_dir: Optional[Path] = None):
    output_dir = get_last_output_dir(output_dir)
    iterations = glob(str(output_dir / "cells/json/*"))
    return [int(Path(iteration).name) for iteration in iterations]


def load_elements_at_iteration(iteration: int, output_dir: Optional[Path] = None):
    output_dir = get_last_output_dir(output_dir)

    elements = []
    for batch_file in glob(str(output_dir / "cells/json/{:020}/*".format(iteration))):
        with open(batch_file) as bf:
            elements += [c["element"][0]["cell"] for c in json.load(bf)["data"]]
    return pd.json_normalize(elements)


def plot_elements_at_iteration(iteration: int, output_dir: Optional[Path] = None):
    elements = load_elements_at_iteration(iteration, output_dir)

    xy = np.array([pos for pos in elements["mechanics.pos"]])
    s = np.array(elements["interaction.sigma"])
    fig, ax = plt.subplots()
    ax.scatter(xy[:, 0], xy[:, 1])

    fig.tight_layout()
    plt.show()


if __name__ == "__main__":
    iterations = get_all_iterations()
    final_iter = sorted(iterations)[-1]
    plot_elements_at_iteration(sorted(iterations)[0])
    plot_elements_at_iteration(final_iter)
