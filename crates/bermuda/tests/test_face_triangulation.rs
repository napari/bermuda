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
