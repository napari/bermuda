import numpy as np
import pytest
from bermuda import (
    triangulate_path_edge,
    triangulate_polygons_face,
    triangulate_polygons_face_3d,
    triangulate_polygons_with_edge,
)


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
        np.array(path, dtype='float32'), limit=3, closed=closed, bevel=bevel
    )
    assert centers.shape == offsets.shape
    assert centers.shape[0] == expected
    assert triangles.shape[0] == expected - 2
    triangles_li = [[int(y) for y in x] for x in triangles]
    assert triangles_li == exp_triangles
    # Verify no NaN values
    assert not np.isnan(centers).any(), 'Centers contain NaN values'
    assert not np.isnan(offsets).any(), 'Offsets contain NaN values'
    # Verify triangle indices are valid
    assert np.all(triangles >= 0), 'Invalid triangle indices'
    assert np.all(triangles < centers.shape[0]), 'Invalid triangle indices'


def test_default_values():
    centers, offsets, triangles = triangulate_path_edge(
        np.array([[0, 0], [0, 10], [10, 10], [10, 0]], dtype='float32')
    )
    assert len(triangles) == 6


def test_default_values_closed():
    centers, offsets, triangles = triangulate_path_edge(
        np.array([[0, 0], [0, 10], [10, 10], [10, 0]], dtype='float32'), True
    )
    assert len(triangles) == 8


def test_default_values_closed_keyword():
    centers, offsets, triangles = triangulate_path_edge(
        np.array([[0, 0], [0, 10], [10, 10], [10, 0]], dtype='float32'),
        closed=True,
    )
    assert len(triangles) == 8


def test_default_values_keyword_order():
    centers, offsets, triangles = triangulate_path_edge(
        np.array([[0, 0], [0, 10], [10, 10], [10, 0]], dtype='float32'),
        bevel=True,
        closed=True,
    )
    assert len(triangles) == 12


def test_change_limit():
    centers, offsets, triangles = triangulate_path_edge(
        np.array([[0, 0], [0, 10], [10, 10], [10, 0]], dtype='float32'),
        bevel=False,
        closed=True,
        limit=0.5,
    )
    assert len(triangles) == 12


TEST_POLYGONS = [
    (np.array(x, dtype=np.float32), y)
    for x, y in [
        ([(0, 0), (1, 1), (0, 2), (2, 1)], [(3, 2, 1), (0, 3, 1)]),
        (
            [(0, 0), (0, 1), (1, 2), (2, 1), (2, 0), (1, 0.5)],
            [(4, 3, 5), (3, 2, 1), (5, 3, 1), (5, 1, 0)],
        ),
        (
            [(0, 1), (0, 2), (1, 1.5), (2, 2), (2, 1), (1, 0.5)],
            [(4, 3, 2), (2, 1, 0), (4, 2, 0), (5, 4, 0)],
        ),
        (
            [(0, 1), (0, 2), (1, 0.5), (2, 2), (2, 1), (1, -0.5)],
            [(2, 1, 0), (2, 0, 5), (4, 3, 2), (5, 4, 2)],
        ),
        ([(0, 0), (1, 2), (2, 0), (1, 1)], [(2, 1, 3), (3, 1, 0)]),
        ([(0, 0), (0, 1), (0.5, 0.5), (1, 0), (1, 1)], [(3, 4, 2), (2, 1, 0)]),
        ([(0, 0), (1, 0), (0.5, 0.5), (0, 1), (1, 1)], [(2, 4, 3), (1, 2, 0)]),
    ]
]


def _renumerate_triangles(polygon, points, triangles):
    """As `py:func:triangulate_polygons_with_edge` change order of points
    we need to fix triangle indices, to be able to compare results.

    Parameters
    ----------
    polygon : list of points
        Original polygon, as list of points
    points : array-like
        Points in order returned by `triangulate_polygons_with_edge`
    triangles : array-like
        Triangles returned by `triangulate_polygons_with_edge`

    Returns
    -------
    list of tuple
        Triangles with fixed indices, as list of tuples of point indices
    """
    point_num = {tuple(point): i for i, point in enumerate(polygon)}
    return [
        tuple(point_num[tuple(points[point])] for point in triangle)
        for triangle in triangles
    ]


@pytest.mark.parametrize(('polygon', 'expected'), TEST_POLYGONS)
def test_triangulate_polygons_with_edge_non_convex(polygon, expected):
    (triangles, points), _edges = triangulate_polygons_with_edge([polygon])
    triangles_ = _renumerate_triangles(polygon, points, triangles)
    assert triangles_ == expected


@pytest.mark.parametrize(('polygon', 'expected'), TEST_POLYGONS)
def test_triangulate_polygons_face_with_edge_non_convex(polygon, expected):
    (triangles, points) = triangulate_polygons_face([polygon])
    triangles_ = _renumerate_triangles(polygon, points, triangles)
    assert triangles_ == expected


@pytest.mark.parametrize('axis', [0, 1, 2])
@pytest.mark.parametrize(('polygon', 'expected'), TEST_POLYGONS)
def test_triangulate_polygons_face_with_edge_non_convex_3d(
    polygon, expected, axis
):
    axes = [0, 1, 2]
    axes.remove(axis)
    polygon_3d = np.zeros((len(polygon), 3), dtype=np.float32)
    polygon_3d[:, axes] = polygon
    polygon_3d[:, axis] = 5

    (triangles, points) = triangulate_polygons_face_3d([polygon_3d])
    triangles_ = _renumerate_triangles(polygon, points[:, axes], triangles)
    assert triangles_ == expected


def test_triangulate_polygon_in_polygon_numpy():
    polygons = [
        np.array([(0, 0), (10, 0), (10, 10), (0, 10)], dtype=np.float32),
        np.array([(4, 4), (6, 4), (6, 6), (4, 6)], dtype=np.float32),
    ]
    (triangles, points), _edges = triangulate_polygons_with_edge(polygons)
    assert len(points) == 8
    assert len(triangles) == 8


def test_triangulate_polygon_with_hole():
    polygon = np.array(
        [
            (0, 0),
            (10, 0),
            (10, 10),
            (0, 10),
            (0, 0),
            (4, 4),
            (6, 4),
            (6, 6),
            (4, 6),
            (4, 4),
        ],
        dtype=np.float32,
    )
    (triangles, points), _edges = triangulate_polygons_with_edge([polygon])
    assert len(points) == 8
    assert len(triangles) == 8


def test_triangulate_polygon_segfault1():
    """Test on polygon that lead to segfault during test"""
    polygon = np.array(
        [
            (205.0625, 1489.83752),
            (204.212509, 1490.4751),
            (204, 1491.11255),
            (202.087509, 1493.45007),
            (201.875, 1494.7251),
            (202.300003, 1496),
            (202.300003, 1498.33752),
            (203.575012, 1499.82507),
            (204.425003, 1500.25),
            (205.0625, 1500.25),
            (205.700012, 1500.67505),
            (206.550003, 1500.67505),
            (207.1875, 1500.25),
            (208.037506, 1500.88757),
            (209.3125, 1499.82507),
            (209.525009, 1499.1875),
            (211.012512, 1497.70007),
            (210.375, 1496.42505),
            (209.525009, 1495.57507),
            (208.462509, 1495.15002),
            (208.675003, 1494.9375),
            (208.462509, 1492.8125),
            (208.037506, 1491.5376),
            (205.912506, 1489.83752),
        ],
        dtype=np.float32,
    )
    triangulate_polygons_with_edge([polygon])


def test_triangulate_polygon_segfault2():
    polygon = np.array(
        [
            [1388.6875, 2744.4375],
            [1388.4751, 2744.6501],
            [1386.5625, 2744.6501],
            [1385.925, 2744.8625],
            [1385.5, 2745.2876],
            [1385.2876, 2747.625],
            [1385.7125, 2748.2627],
            [1385.7125, 2749.1125],
            [1386.1376, 2749.75],
            [1389.9625, 2753.7876],
            [1390.3876, 2754.6377],
            [1391.025, 2754.6377],
            [1392.0875, 2753.1501],
            [1392.3, 2753.3625],
            [1392.3, 2754.6377],
            [1392.5126, 2754.2126],
            [1392.3, 2754.0],
            [1392.3, 2751.4502],
            [1392.7251, 2750.3877],
            [1391.6626, 2748.9001],
            [1391.6626, 2747.4126],
            [1390.8125, 2745.5],
            [1390.175, 2745.2876],
            [1389.3251, 2744.4375],
        ],
        dtype=np.float32,
    )
    triangulate_polygons_with_edge([polygon])


def test_triangulate_polygon_segfault3():
    polygon = np.array(
        [
            (1066.32507, 1794.3501),
            (1065.6875, 1794.77502),
            (1063.77502, 1794.77502),
            (1063.5625, 1794.98755),
            (1063.13757, 1794.98755),
            (1062.28748, 1795.83752),
            (1062.28748, 1797.32507),
            (1062.07507, 1797.5376),
            (1062.28748, 1797.5376),
            (1062.5, 1797.75),
            (1063.13757, 1797.75),
            (1063.5625, 1797.32507),
            (1064.625, 1797.32507),
            (1065.26257, 1797.96252),
            (1064.83752, 1797.5376),
            (1064.83752, 1796.90002),
            (1065.47498, 1796.26257),
            (1065.47498, 1796.05005),
            (1065.6875, 1795.83752),
            (1066.53748, 1795.83752),
            (1066.75, 1795.625),
            (1066.75, 1794.98755),
            (1066.96252, 1794.77502),
            (1066.75, 1794.3501),
        ],
        dtype=np.float32,
    )
    triangulate_polygons_with_edge([polygon])


def test_triangulate_polygon_segfault4():
    polygon = np.array(
        [
            [657.6875, 2280.975],
            [657.6875, 2281.6125],
            [657.05005, 2282.25],
            [656.2, 2284.1626],
            [657.6875, 2285.8625],
            [658.11255, 2286.7126],
            [659.8125, 2288.4126],
            [659.8125, 2288.625],
            [661.9375, 2290.5376],
            [662.78754, 2290.9626],
            [664.7, 2292.2375],
            [665.3375, 2292.2375],
            [665.97504, 2291.175],
            [666.61255, 2290.5376],
            [666.61255, 2289.6875],
            [666.1875, 2288.625],
            [664.48755, 2286.925],
            [664.0625, 2286.925],
            [663.2125, 2286.5],
            [661.9375, 2284.8],
            [660.66254, 2284.8],
            [660.45, 2284.5876],
            [660.45, 2284.1626],
            [657.6875, 2281.1875],
        ],
        dtype=np.float32,
    )
    triangulate_polygons_with_edge([polygon])


def test_triangulate_polygon_segfault5():
    polygon = np.array(
        [
            [895.05005, 2422.5],
            [894.8375, 2422.7126],
            [894.41254, 2422.7126],
            [893.98755, 2423.1375],
            [893.35004, 2423.1375],
            [892.92505, 2423.5625],
            [892.7125, 2423.5625],
            [891.4375, 2424.8376],
            [891.22504, 2424.8376],
            [892.075, 2424.8376],
            [892.5, 2425.05],
            [893.35004, 2425.9001],
            [893.775, 2426.75],
            [893.775, 2427.3875],
            [894.625, 2426.75],
            [895.6875, 2426.75],
            [896.11255, 2426.1125],
            [896.11255, 2425.05],
            [895.9, 2424.8376],
            [896.53754, 2423.7751],
            [896.53754, 2423.35],
            [896.75, 2422.925],
            [896.11255, 2422.7126],
            [895.9, 2422.5],
        ],
        dtype=np.float32,
    )
    triangulate_polygons_with_edge([polygon])


def test_triangulate_segment():
    polygon = np.array([[0, 0], [10, 10]], dtype=np.float32)
    (
        (face_triangles, face_vertices),
        (edge_centers, edge_offsets, edge_triangles),
    ) = triangulate_polygons_with_edge([polygon])
    assert len(edge_centers) == 8
    assert len(edge_offsets) == 8
    assert len(edge_triangles) == 6
    assert len(face_vertices) == 2
    assert len(face_triangles) == 1


def test_face_triangulate_segment():
    polygon = np.array([[0, 0], [10, 10]], dtype=np.float32)
    (face_triangles, face_vertices) = triangulate_polygons_face([polygon])
    assert len(face_vertices) == 2
    assert len(face_triangles) == 1


def test_edge_triangulate_segment():
    polygon = np.array([[0, 0], [10, 10]], dtype=np.float32)
    (edge_centers, edge_offsets, edge_triangles) = triangulate_path_edge(
        polygon, closed=False
    )
    assert len(edge_centers) == 4
    assert len(edge_offsets) == 4
    assert len(edge_triangles) == 2


def test_edge_triangulate_segment_closed():
    polygon = np.array([[0, 0], [10, 10]], dtype=np.float32)
    (edge_centers, edge_offsets, edge_triangles) = triangulate_path_edge(
        polygon, closed=True
    )
    assert len(edge_centers) == 8
    assert len(edge_offsets) == 8
    assert len(edge_triangles) == 6
