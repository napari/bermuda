use rstest::rstest;
use std::collections::HashSet;
use triangulation::intersection;
use triangulation::point::{Point, Segment};

fn seg(x1: i32, y1: i32, x2: i32, y2: i32) -> Segment {
    Segment::new(
        Point::new(x1 as f32, y1 as f32),
        Point::new(x2 as f32, y2 as f32),
    )
}

fn seg_f(x1: f32, y1: f32, x2: f32, y2: f32) -> Segment {
    Segment::new(Point::new(x1, y1), Point::new(x2, y2))
}

#[rstest]
#[case(seg(0, 0, 2, 2), Point::new(1.0, 1.0), true)]
#[case(seg(0, 0, 0, 2), Point::new(0.0, 1.0), true)]
#[case(seg(0, 0, 2, 0), Point::new(1.0, 0.0), true)]
#[case(seg(0, 0, 1, 1), Point::new(2.0, 2.0), false)]
#[case(seg(0, 0, 0, 1), Point::new(0.0, 2.0), false)]
#[case(seg(0, 0, 1, 0), Point::new(2.0, 0.0), false)]
#[case(seg_f(1e6, 1e6, 3e6, 3e6), Point::new(2e6, 2e6), true)]
#[case(seg_f(0.0, 0.0, 0.0001, 0.0001), Point::new(0.00005, 0.00005), true)]
#[case(seg(0, 0, -2, -2), Point::new(-1.0, -1.0), true)]
fn test_on_segment_if_collinear(#[case] s: Segment, #[case] q: Point, #[case] expected: bool) {
    assert_eq!(intersection::on_segment_if_collinear(&s, q), expected);
}

#[rstest]
fn test_do_intersect_crossing_segments() {
    assert!(intersection::do_intersect(
        &Segment::new(Point::new_i(0, -1), Point::new_i(0, 1)),
        &Segment::new(Point::new_i(-1, 0), Point::new_i(1, 0))
    ));
}

#[rstest]
#[case(Segment::new_i((0, 0), (1, 1)), Segment::new_i((1, 0), (0, 1)))]
#[case(Segment::new_i((1, 0), (0, 1)), Segment::new_i((0, 0), (1, 1)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_i((0, 1), (1, 1)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_i((1, 1), (0, 1)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_i((0, 0), (1, 1)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_i((1, 1), (0, 0)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_f((0.0, 0.5), (1.0, 1.0)))]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_f((1.0, 1.0), (0.0, 0.5)))]
fn test_do_intersect(#[case] s1: Segment, #[case] s2: Segment) {
    assert!(intersection::do_intersect(&s1, &s2));
}

#[rstest]
#[case(Segment::new_i((0, 0), (0, 1)), Segment::new_i((1, 2), (-1, 2)))]
#[case(Segment::new_i((0, 0), (1, 0)), Segment::new_i((2, 1), (2, -1)))]
#[case(Segment::new_i((0, 0), (1, 1)), Segment::new_i((1, 2), (0, 1)))]
fn test_do_intersect_ne(#[case] s1: Segment, #[case] s2: Segment) {
    assert!(!intersection::do_intersect(&s1, &s2));
}

#[rstest]
fn test_do_intersect_parallel_segments() {
    assert!(!intersection::do_intersect(
        &Segment::new(Point::new_i(0, -1), Point::new_i(0, 1)),
        &Segment::new(Point::new_i(1, -2), Point::new_i(1, 1))
    ))
}

#[rstest]
#[case(Segment::new_i((0, 0), (2, 2)), Segment::new_i((2, 0), (0, 2)), Point::new_i(1, 1))]
#[case(Segment::new_i((0, 0), (1, 0)), Segment::new_i((0, 1), (0, 0)), Point::new_i(0, 0))]
#[case(Segment::new_i((0, 0), (2, 0)), Segment::new_i((1, 0), (1, 2)), Point::new_i(1, 0))]
#[case(Segment::new_f((0.0, 0.0), (2.0, 2.0)), Segment::new_f((2.0, 0.0), (0.0, 2.0)), Point::new(1.0, 1.0))]
#[case(Segment::new_f((0.0, 0.0), (1.0, 1.0)), Segment::new_f((0.99, 0.0), (0.0, 0.99)), Point::new(0.495, 0.495))]
#[case(Segment::new_f((1e6, 1e6), (2e6, 2e6)), Segment::new_f((2e6, 1e6), (1e6, 2e6)), Point::new(1.5e6, 1.5e6))]
fn test_find_intersection_point(#[case] s1: Segment, #[case] s2: Segment, #[case] expected: Point) {
    assert_eq!(
        intersection::find_intersection(&s1, &s2),
        intersection::Intersection::PointIntersection(expected)
    );
    assert_eq!(
        intersection::find_intersection(&s2, &s1),
        intersection::Intersection::PointIntersection(expected)
    );
}

#[rstest]
fn test_find_intersection_collinear_segments() {
    assert_eq!(
        intersection::find_intersection(
            &Segment::new_i((0, 0), (2, 0)),
            &Segment::new_i((1, 0), (3, 0))
        ),
        intersection::Intersection::CollinearWithOverlap((Point::new_i(1, 0), Point::new_i(2, 0)))
    );
    assert_eq!(
        intersection::find_intersection(
            &Segment::new_i((0, 0), (2, 0)),
            &Segment::new_i((1, 0), (3, 0))
        ),
        intersection::Intersection::CollinearWithOverlap((Point::new_i(1, 0), Point::new_i(2, 0)))
    );
}

/// Tests a simple square configuration with no intersections.
/// Each segment connects to the next at endpoints, but there
/// are no true intersections between non-adjacent segments.
///  (1, 0) --- (1, 1)
///   |           |
/// (0, 0) --- (0, 1)
#[rstest]
fn test_find_intersections_1() {
    let segments = vec![
        Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
        Segment::new(Point::new(0.0, 1.0), Point::new(1.0, 1.0)),
        Segment::new(Point::new(1.0, 1.0), Point::new(1.0, 0.0)),
        Segment::new(Point::new(1.0, 0.0), Point::new(0.0, 0.0)),
    ];
    let intersections = intersection::find_intersections(&segments);
    assert!(intersections.is_empty());
}

/// Tests a configuration with two intersecting diagonals.
/// Expected behavior:
/// - Only one intersection is recorded between segments 1 and 3
/// - The intersection occurs at (0.5, 0.5)
///     (1, 0) --- (1, 1)
///         \     /
///          \   /
///           \ /
///            X
///           / \
///          /   \
///         /     \
///     (0, 0) --- (0, 1)
#[rstest]
fn test_find_intersections_2() {
    let segments = vec![
        Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
        Segment::new(Point::new(0.0, 1.0), Point::new(1.0, 0.0)),
        Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0)),
        Segment::new(Point::new(1.0, 1.0), Point::new(0.0, 0.0)),
        Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)),
    ];
    let intersections = intersection::find_intersections(&segments);
    let expected = [(1, 3)]
        .iter()
        .map(|&(a, b)| intersection::OrderedPair::new(a, b))
        .collect();
    assert_eq!(intersections, expected);
}

#[rstest]
#[case::no_intersections_simple_square(
    vec![
        Segment::new_i((0, 0), (0, 1)),
        Segment::new_i((0, 1), (1, 1)),
        Segment::new_i((1, 1), (1, 0)),
        Segment::new_i((1, 0), (0, 0)),
    ],
    HashSet::new()
)]
#[case::one_intersection_crossing_diagonals(
    vec![
        Segment::new_i((0, 0), (2, 2)),
        Segment::new_i((2, 0), (0, 2)),
    ],
    [(0, 1)].iter().map(|&(a, b)| intersection::OrderedPair::new(a, b)).collect()
)]
#[case::multiple_intersections_complex_shape(
    vec![
        Segment::new_i((0, 0), (2, 2)),
        Segment::new_i((2, 0), (0, 2)),
        Segment::new_i((1, 0), (1, 2)),
        Segment::new_i((0, 1), (2, 1)),
    ],
    [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)].iter().map(|&(a, b)| intersection::OrderedPair::new(a, b)).collect()
)]
#[case::no_intersections_non_intersecting_lines(
    vec![
        Segment::new_i((0, 0), (1, 1)),
        Segment::new_i((2, 2), (3, 3)),
    ],
    HashSet::new()
)]
#[case::one_intersection_t_shaped_intersection(
    vec![
        Segment::new_i((0, 0), (2, 0)),
        Segment::new_i((1, -1), (1, 1)),
    ],
    [(0, 1)].iter().map(|&(a, b)| intersection::OrderedPair::new(a, b)).collect()
)]
#[case::multiple_intersections_grid_shape(
    vec![
        Segment::new_i((0, 0), (2, 0)),
        Segment::new_i((0, 1), (2, 1)),
        Segment::new_i((0, 2), (2, 2)),
        Segment::new_i((0, 0), (0, 2)),
        Segment::new_i((1, 0), (1, 2)),
        Segment::new_i((2, 0), (2, 2)),
    ],
    [(0, 4), (1, 3), (1, 4), (1, 5), (2, 4)].iter().map(|&(a, b)| intersection::OrderedPair::new(a, b)).collect()
)]
fn test_find_intersections_param(
    #[case] segments: Vec<Segment>,
    #[case] expected: HashSet<intersection::OrderedPair>,
) {
    assert_eq!(intersection::find_intersections(&segments), expected);
}

#[rstest]
fn test_find_intersection_points_cross() {
    /*
    (1, 0) --- (1, 1)
        \     /
         \   /
          \ /
           X
          / \
         /   \
        /     \
    (0, 0) --- (0, 1)
    */
    let polygon = vec![vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(0.0, 1.0),
    ]];

    let result = intersection::find_intersection_points(&polygon);
    let expected = vec![vec![
        Point::new(0.0, 0.0),
        Point::new(0.5, 0.5),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(0.5, 0.5),
        Point::new(0.0, 1.0),
    ]];

    assert_eq!(result, expected);
}

#[rstest]
fn test_find_intersection_points_cross_intersect_in_point() {
    /*
    (1, 0) --- (1, 1)
        \     /
         \   /
          \ /
           X
          / \
         /   \
        /     \
    (0, 0) --- (0, 1)
    */
    let polygon = vec![vec![
        Point::new(0.0, 0.0),
        Point::new(0.5, 0.5),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(0.0, 1.0),
    ]];

    let result = intersection::find_intersection_points(&polygon);
    let expected = vec![vec![
        Point::new(0.0, 0.0),
        Point::new(0.5, 0.5),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(0.5, 0.5),
        Point::new(0.0, 1.0),
    ]];

    assert_eq!(result, expected);
}

#[rstest]
fn test_find_intersection_points_overleap_edge() {
    let polygon_list = vec![
        vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 3.0),
            Point::new(0.0, 3.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(1.0, 1.0),
        ],
    ];
    let result = intersection::find_intersection_points(&polygon_list);
    assert_eq!(result.len(), 2);
    assert_eq!(
        result[0],
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 3.0),
            Point::new(0.0, 3.0)
        ]
    );
    assert_eq!(result[1], polygon_list[1]);
}

#[rstest]
#[case::rectangle(vec![vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)]], 1, 4)]
#[case::rectangle_in_rectangle(vec![vec![Point::new(0.0, 0.0), Point::new(3.0, 0.0), Point::new(3.0, 3.0), Point::new(0.0, 3.0), Point::new(0.0, 0.0), Point::new(1.0, 1.0),
    Point::new(2.0, 1.0),
    Point::new(2.0, 2.0),
    Point::new(1.0, 2.0),
    Point::new(1.0, 1.0),]], 2, 8)]
fn test_split_polygon_on_repeated_edges(
    #[case] polygon_list: Vec<Vec<Point>>,
    #[case] polygon_count: usize,
    #[case] edge_count: usize,
) {
    let (polygons, edges) = intersection::split_polygons_on_repeated_edges(&polygon_list);
    assert_eq!(polygons.len(), polygon_count);
    assert_eq!(edges.len(), edge_count);
}

#[rstest]
fn test_split_polygon_on_repeated_edges_merge_two_polygons() {
    let polygon_list = vec![
        vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 3.0),
            Point::new(0.0, 3.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(1.0, 1.0),
        ],
    ];
    let (polygons, edges) = intersection::split_polygons_on_repeated_edges(&polygon_list);
    assert_eq!(polygons.len(), 1);
    assert_eq!(edges.len(), 8);
}
