from typing import Literal

import numpy as np
import numpy.typing as npt

def triangulate_path_edge(
    path: npt.NDArray[tuple[int, Literal[2]], np.float32],
    closed: bool = False,
    limit: float = 3.0,
    bevel: bool = False,
) -> tuple[
    npt.NDArray[tuple[int, Literal[2]], np.float32],
    npt.NDArray[tuple[int, Literal[2]], np.float32],
    npt.NDArray[tuple[int, Literal[3]], np.uint32],
]: ...
def triangulate_polygons_with_edge(
    polygons: list[npt.NDArray[tuple[int, Literal[2]], np.float32]],
) -> tuple[
    tuple[
        npt.NDArray[tuple[int, Literal[2]], np.float32],
        npt.NDArray[tuple[int, Literal[2]], np.float32],
        npt.NDArray[tuple[int, Literal[3]], np.uint32],
    ],
    tuple[
        npt.NDArray[tuple[int, Literal[3]], np.uint32],
        npt.NDArray[tuple[int, Literal[2]], np.float32],
    ],
]: ...
def triangulate_polygons_face(
    polygons: list[npt.NDArray[tuple[int, Literal[2]], np.float32]],
) -> tuple[
    npt.NDArray[tuple[int, Literal[3]], np.uint32],
    npt.NDArray[tuple[int, Literal[2]], np.float32],
]: ...
def triangulate_polygons_face_3d(
    polygons: list[npt.NDArray[tuple[int, Literal[3]], np.float32]],
) -> tuple[
    npt.NDArray[tuple[int, Literal[3]], np.uint32],
    npt.NDArray[tuple[int, Literal[3]], np.float32],
]: ...
