use rstest::rstest;

use std::collections::HashMap;
use std::collections::HashSet;
use triangulation::face_triangulation::sweeping_line_triangulation;
use triangulation::point::{calc_dedup_edges, Point, Triangle};

const DIAMOND: [Point; 4] = [
    Point::new(1.0, 0.0),
    Point::new(2.0, 1.0),
    Point::new(1.0, 2.0),
    Point::new(0.0, 1.0),
];

// #[rstest]
// fn test_diamond() {
//     let (triangles, points) =
//         sweeping_line_triangulation(calc_dedup_edges(&vec![DIAMOND.to_vec()]));
//     assert_eq!(triangles.len(), 2);
//     assert_eq!(points.len(), 4);
//     assert_eq!(
//         points.into_iter().collect::<HashSet<_>>(),
//         DIAMOND.iter().cloned().collect::<HashSet<_>>()
//     );
// }

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
// #[case::pentagon_1(
//     vec![
//         Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(0.5, 0.5),
//         Point::new(1.0, 0.0), Point::new(1.0, 1.0)
//     ],
//     vec![[3, 4, 2], [2, 1, 0]]
// )]
// #[case::pentagon_2(
//     vec![
//         Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.5, 0.5),
//         Point::new(0.0, 1.0), Point::new(1.0, 1.0)
//     ],
//     vec![[2, 4, 3], [1, 2, 0]]
// )]
fn test_triangulate_polygon_non_convex(
    #[case] polygon: Vec<Point>,
    #[case] expected: Vec<[usize; 3]>,
) {
    let (triangles, points) = sweeping_line_triangulation(calc_dedup_edges(&vec![polygon.clone()]));
    let triangles_ = renumerate_triangles(&polygon, &points, &triangles);
    assert_eq!(triangles_, expected);
}
