use rstest::rstest;

use std::collections::HashMap;
use std::collections::HashSet;
use triangulation::face_triangulation::sweeping_line_triangulation;
use triangulation::point::{calc_dedup_edges, Point, Triangle};
use triangulation::split_polygons_on_repeated_edges;

const DIAMOND: [Point; 4] = [
    Point::new(1.0, 0.0),
    Point::new(2.0, 1.0),
    Point::new(1.0, 2.0),
    Point::new(0.0, 1.0),
];

#[rstest]
fn test_diamond() {
    let (triangles, points) =
        sweeping_line_triangulation(calc_dedup_edges(&vec![DIAMOND.to_vec()]));
    assert_eq!(triangles.len(), 2);
    assert_eq!(points.len(), 4);
    assert_eq!(
        points.into_iter().collect::<HashSet<_>>(),
        DIAMOND.iter().cloned().collect::<HashSet<_>>()
    );
}

fn renumerate_triangles(
    polygon: &[Point],
    points: &[Point],
    triangles: &[Triangle],
) -> Vec<[usize; 3]> {
    let point_num: HashMap<Point, usize> =
        polygon.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    triangles
        .iter()
        .map(|t| {
            [
                point_num[&points[t.x as usize]],
                point_num[&points[t.y as usize]],
                point_num[&points[t.z as usize]],
            ]
        })
        .collect()
}

#[rstest]
#[case::square_with_diagonal(
    vec![Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 2.0), Point::new(2.0, 1.0)],
    vec![[3, 2, 1], [0, 3, 1]]
)]
#[case::complex_hexagon(
    vec![
        Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 2.0),
        Point::new(2.0, 1.0), Point::new(2.0, 0.0), Point::new(1.0, 0.5)
    ],
    vec![[4, 3, 5], [3, 2, 1], [5, 3, 1], [5, 1, 0]]
)]
#[case::irregular_hexagon(
    vec![
        Point::new(0.0, 1.0), Point::new(0.0, 2.0), Point::new(1.0, 1.5),
        Point::new(2.0, 2.0), Point::new(2.0, 1.0), Point::new(1.0, 0.5)
    ],
    vec![[4, 3, 2], [2, 1, 0], [4, 2, 0], [5, 4, 0]]
)]
#[case::irregular_hexagon_2(
    vec![
        Point::new(0.0, 1.0), Point::new(0.0, 2.0), Point::new(1.0, 0.5),
        Point::new(2.0, 2.0), Point::new(2.0, 1.0), Point::new(1.0, -0.5)
    ],
    vec![[2, 1, 0], [2, 0, 5], [4, 3, 2], [5, 4, 2]]
)]
#[case::triangle_with_interior(
    vec![
        Point::new(0.0, 0.0), Point::new(1.0, 2.0), Point::new(2.0, 0.0),
        Point::new(1.0, 1.0)
    ],
    vec![[2, 1, 3], [3, 1, 0]]
)]
#[case::pentagon_1(
    vec![
        Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(0.5, 0.5),
        Point::new(1.0, 0.0), Point::new(1.0, 1.0)
    ],
    vec![[3, 4, 2], [2, 1, 0]]
)]
#[case::pentagon_2(
    vec![
        Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.5, 0.5),
        Point::new(0.0, 1.0), Point::new(1.0, 1.0)
    ],
    vec![[2, 4, 3], [1, 2, 0]]
)]
fn test_triangulate_polygon_non_convex(
    #[case] polygon: Vec<Point>,
    #[case] expected: Vec<[usize; 3]>,
) {
    let (new_polygons, segments) = split_polygons_on_repeated_edges(&vec![polygon.clone()]);
    assert_eq!(
        new_polygons[0].iter().cloned().collect::<HashSet<_>>(),
        polygon.iter().cloned().collect::<HashSet<_>>()
    );
    let (triangles, points) = sweeping_line_triangulation(segments);
    let triangles_ = renumerate_triangles(&polygon, &points, &triangles);
    assert_eq!(triangles_, expected);
}

#[rstest]
fn test_triangulate_polygon_segfault1() {
    //Test on polygon that lead to segfault during test
    let polygon = vec![
        Point::new(205.0625, 1489.83752),
        Point::new(204.212509, 1490.4751),
        Point::new(204.0, 1491.11255),
        Point::new(202.087509, 1493.45007),
        Point::new(201.875, 1494.7251),
        Point::new(202.300003, 1496.0),
        Point::new(202.300003, 1498.33752),
        Point::new(203.575012, 1499.82507),
        Point::new(204.425003, 1500.25),
        Point::new(205.0625, 1500.25),
        Point::new(205.700012, 1500.67505),
        Point::new(206.550003, 1500.67505),
        Point::new(207.1875, 1500.25),
        Point::new(208.037506, 1500.88757),
        Point::new(209.3125, 1499.82507),
        Point::new(209.525009, 1499.1875),
        Point::new(211.012512, 1497.70007),
        Point::new(210.375, 1496.42505),
        Point::new(209.525009, 1495.57507),
        Point::new(208.462509, 1495.15002),
        Point::new(208.675003, 1494.9375),
        Point::new(208.462509, 1492.8125),
        Point::new(208.037506, 1491.5376),
        Point::new(205.912506, 1489.83752),
    ];
    let (_new_polygons, segments) = split_polygons_on_repeated_edges(&vec![polygon]);
    sweeping_line_triangulation(segments);
}

#[rstest]
fn test_triangulate_polygon_country() {
    let polygon_ = vec![
        [-28.58, 196.34],
        [-28.08, 196.82],
        [-28.36, 197.22],
        [-28.78, 197.39],
        [-28.86, 197.84],
        [-29.05, 198.46],
        [-28.97, 199.],
        [-28.46, 199.89],
        [-24.77, 199.9],
        [-24.92, 200.17],
        [-25.87, 200.76],
        [-26.48, 200.67],
        [-26.83, 200.89],
        [-26.73, 201.61],
        [-26.28, 202.11],
        [-25.98, 202.58],
        [-25.5, 202.82],
        [-25.27, 203.31],
        [-25.39, 203.73],
        [-25.67, 204.21],
        [-25.72, 205.03],
        [-25.49, 205.66],
        [-25.17, 205.77],
        [-24.7, 205.94],
        [-24.62, 206.49],
        [-24.24, 206.79],
        [-23.57, 207.12],
        [-22.83, 208.02],
        [-22.09, 209.43],
        [-22.1, 209.84],
        [-22.27, 210.32],
        [-22.15, 210.66],
        [-22.25, 211.19],
        [-23.66, 211.67],
        [-24.37, 211.93],
        [-25.48, 211.75],
        [-25.84, 211.84],
        [-25.66, 211.33],
        [-25.73, 211.04],
        [-26.02, 210.95],
        [-26.4, 210.68],
        [-26.74, 210.69],
        [-27.29, 211.28],
        [-27.18, 211.87],
        [-26.73, 212.07],
        [-26.74, 212.83],
        [-27.47, 212.58],
        [-28.3, 212.46],
        [-28.75, 212.2],
        [-29.26, 211.52],
        [-29.4, 211.33],
        [-29.91, 210.9],
        [-30.42, 210.62],
        [-31.14, 210.06],
        [-32.17, 208.93],
        [-32.77, 208.22],
        [-33.23, 207.46],
        [-33.61, 206.42],
        [-33.67, 205.91],
        [-33.94, 205.78],
        [-33.8, 205.17],
        [-33.99, 204.68],
        [-33.79, 203.59],
        [-33.92, 202.99],
        [-33.86, 202.57],
        [-34.26, 201.54],
        [-34.42, 200.69],
        [-34.8, 200.07],
        [-34.82, 199.62],
        [-34.46, 199.19],
        [-34.44, 198.86],
        [-34., 198.42],
        [-34.14, 198.38],
        [-33.87, 198.24],
        [-33.28, 198.25],
        [-32.61, 197.93],
        [-32.43, 198.25],
        [-31.66, 198.22],
        [-30.73, 197.57],
        [-29.88, 197.06],
        [-29.88, 197.06],
        [-28.58, 196.34],
        [-28.96, 208.98],
        [-28.65, 208.54],
        [-28.85, 208.07],
        [-29.24, 207.53],
        [-29.88, 207.],
        [-30.65, 207.75],
        [-30.55, 208.11],
        [-30.23, 208.29],
        [-30.07, 208.85],
        [-29.74, 209.02],
        [-29.26, 209.33],
        [-28.96, 208.98],
    ];
    let polygon = polygon_
        .iter()
        .map(|&p| Point::new(p[0], p[1]))
        .collect::<Vec<_>>();
    let (_new_polygons, segments) = split_polygons_on_repeated_edges(&vec![polygon]);
    assert_eq!(_new_polygons.len(), 2);
    sweeping_line_triangulation(segments);
}
