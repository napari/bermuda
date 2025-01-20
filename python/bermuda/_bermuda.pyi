import numpy as np

def triangulate_path_edge(
    path: np.ndarray, closed: bool, limit: float, bevel: bool
) -> tuple[np.ndarray, np.ndarray, np.ndarray]: ...
