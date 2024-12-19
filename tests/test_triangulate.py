import numpy as np
import pytest
from bermuda import triangulate_path_edge


@pytest.mark.parametrize(
    ('path', 'closed', 'bevel', 'expected', 'exp_triangles'),
    [
        (
            [[0, 0], [0, 10], [10, 10], [10, 0]],
            True,
            False,
            10,
            [
                [2, 1, 0],
                [1, 2, 3],
                [4, 3, 2],
                [3, 4, 5],
                [6, 5, 4],
                [5, 6, 7],
                [8, 7, 6],
                [7, 8, 9],
            ],
        ),
        (
            [[0, 0], [0, 10], [10, 10], [10, 0]],
            False,
            False,
            8,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [3, 4, 5], [6, 5, 4], [5, 6, 7]],
        ),
        (
            [[0, 0], [0, 10], [10, 10], [10, 0]],
            True,
            True,
            14,
            [
                [2, 1, 0],
                [3, 2, 0],
                [2, 3, 4],
                [5, 4, 3],
                [6, 5, 3],
                [5, 6, 7],
                [8, 7, 6],
                [9, 8, 6],
                [8, 9, 10],
                [11, 10, 9],
                [12, 11, 9],
                [11, 12, 13],
            ],
        ),
        (
            [[0, 0], [0, 10], [10, 10], [10, 0]],
            False,
            True,
            10,
            [
                [2, 1, 0],
                [1, 2, 3],
                [4, 3, 2],
                [5, 4, 2],
                [4, 5, 6],
                [7, 6, 5],
                [8, 7, 5],
                [7, 8, 9],
            ],
        ),
        (
            [[2, 10], [0, -5], [-2, 10], [-2, -10], [2, -10]],
            True,
            False,
            15,
            [
                [2, 1, 0],
                [1, 2, 3],
                [1, 3, 4],
                [5, 4, 3],
                [6, 5, 3],
                [5, 6, 7],
                [8, 7, 6],
                [7, 8, 9],
                [7, 9, 10],
                [11, 10, 9],
                [10, 11, 12],
                [13, 12, 11],
                [12, 13, 14],
            ],
        ),
        ([[0, 0], [0, 10]], False, False, 4, [[2, 1, 0], [1, 2, 3]]),
        (
            [[0, 0], [0, 10], [0, 20]],
            False,
            False,
            6,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [3, 4, 5]],
        ),
        (
            [[0, 0], [0, 2], [10, 1]],
            True,
            False,
            9,
            [
                [2, 1, 0],
                [1, 2, 3],
                [4, 3, 2],
                [3, 4, 5],
                [6, 5, 4],
                [7, 6, 4],
                [6, 7, 8],
            ],
        ),
        (
            [[0, 0], [10, 1], [9, 1.1]],
            False,
            False,
            7,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [3, 4, 5], [3, 5, 6]],
        ),
        (
            [[9, 0.9], [10, 1], [0, 2]],
            False,
            False,
            7,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [3, 4, 5], [3, 5, 6]],
        ),
        (
            [[0, 0], [-10, 1], [-9, 1.1]],
            False,
            False,
            7,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [5, 4, 2], [4, 5, 6]],
        ),
        (
            [[-9, 0.9], [-10, 1], [0, 2]],
            False,
            False,
            7,
            [[2, 1, 0], [1, 2, 3], [4, 3, 2], [5, 4, 2], [4, 5, 6]],
        ),
    ],
)
def test_triangulate_path_edge_py(
    path, closed, bevel, expected, exp_triangles
):
    centers, offsets, triangles = triangulate_path_edge(
        np.array(path, dtype='float32'), closed=closed, bevel=bevel
    )
    assert centers.shape == offsets.shape
    assert centers.shape[0] == expected
    assert triangles.shape[0] == expected - 2
    triangles_li = [[int(y) for y in x] for x in triangles]
    assert triangles_li == exp_triangles
