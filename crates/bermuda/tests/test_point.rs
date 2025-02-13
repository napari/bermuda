use rstest::rstest;
use std::collections::HashSet;
use triangulation::point::{
    calc_dedup_edges, orientation, split_polygon_on_repeated_edges, Orientation, Point, Segment,
    Vector,
};

#[rstest]
fn test_segment_order() {
    let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(0.0, 0.0));
    assert_eq!(s1, s2);
}

#[rstest]
#[case::base(1.0, 0.0, 1.0, 1.0, 2.0, 1.0)]
#[case::zero_vector(0.0, 0.0, 1.0, 1.0, 1.0, 1.0)] // zero vector
#[case::negative_vector(1.0, 1.0, -1.0, -1.0, 0.0, 0.0)] // negative vector
#[case::larger_components(10.0, 20.0, 30.0, 40.0, 40.0, 60.0)] // larger components
fn test_vector_add(
    #[case] x1: f32,
    #[case] y1: f32,
    #[case] x2: f32,
    #[case] y2: f32,
    #[case] expected_x: f32,
    #[case] expected_y: f32,
) {
    assert_eq!(
        Point::new(x1, y1) + Vector::new(x2, y2),
        Point::new(expected_x, expected_y)
    );
}

#[rstest]
#[case::colinear_1(
    Point::new(0.0, 0.0),
    Point::new(0.0, 1.0),
    Point::new(0.0, 2.0),
    Orientation::Collinear
)]
#[case::colinear_2(
    Point::new(0.0, 0.0),
    Point::new(0.0, 2.0),
    Point::new(0.0, 1.0),
    Orientation::Collinear
)]
#[case::colinear_3(
    Point::new(0.0, 2.0),
    Point::new(0.0, 0.0),
    Point::new(0.0, 1.0),
    Orientation::Collinear
)]
#[case::clockwise_1(
    Point::new(0.0, 0.0),
    Point::new(0.0, 1.0),
    Point::new(1.0, 2.0),
    Orientation::Clockwise
)]
#[case::counter_clockwise_1(Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(-1.0, 2.0), Orientation::CounterClockwise)]
#[case::counter_clockwise_2(
    Point::new(0.0, 0.0),
    Point::new(1.0, 0.0),
    Point::new(1.0, 1.0),
    Orientation::CounterClockwise
)] // Right angle
#[case::colinear_4(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, -1.0), Orientation::Collinear)] // Same x, not collinear
#[case::counter_clockwise_precision(Point::new(0.0, 0.0), Point::new(0.0001, 0.0001), Point::new(-0.0001, 0.0001), Orientation::CounterClockwise)] // Precision case1
fn test_orientation(
    #[case] p: Point,
    #[case] q: Point,
    #[case] r: Point,
    #[case] expected: Orientation,
) {
    assert_eq!(orientation(p, q, r), expected);
}

#[rstest]
#[case::rectangle(vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)], 1, 4)]
#[case::rectangle_in_rectangle(vec![Point::new(0.0, 0.0), Point::new(3.0, 0.0), Point::new(3.0, 3.0), Point::new(0.0, 3.0), Point::new(0.0, 0.0), Point::new(1.0, 1.0),
    Point::new(2.0, 1.0),
    Point::new(2.0, 2.0),
    Point::new(1.0, 2.0),
    Point::new(1.0, 1.0),], 2, 8)]
fn test_split_polygon_on_repeated_edges(
    #[case] polygon: Vec<Point>,
    #[case] polygon_count: usize,
    #[case] edge_count: usize,
) {
    let (polygons, edges) = split_polygon_on_repeated_edges(&polygon);
    assert_eq!(polygons.len(), polygon_count);
    assert_eq!(edges.len(), edge_count);
}

#[rstest]
#[case::empty_list(&[], 0)]
#[case::single_triangle(
    &[vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(0.0, 1.0),
    ]],
    3
)]
#[case::two_adjacent_triangles(
    &[
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(0.0, 1.0),
        ],
    ],
    4
)]
#[case::rectangle_with_shared_edge(
    &[
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(2.0, 1.0),
            Point::new(1.0, 1.0),
        ],
    ],
    6
)]
fn test_calc_dedup_edges(#[case] input: &[Vec<Point>], #[case] expected_edge_count: usize) {
    let result = calc_dedup_edges(input);
    assert_eq!(result.len(), expected_edge_count);
}

#[rstest]
fn test_edge_deduplication() {
    // Create two triangles that share an edge
    let polygon1 = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(0.5, 1.0),
    ];
    let polygon2 = vec![
        Point::new(1.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(0.5, 1.0),
    ];

    let edges = calc_dedup_edges(&[polygon1, polygon2]);

    // The shared edge should be removed (appears twice)
    let expected_edges = {
        let mut set = HashSet::new();
        set.insert(Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0)));
        set.insert(Segment::new(Point::new(0.0, 0.0), Point::new(0.5, 1.0)));
        set.insert(Segment::new(Point::new(1.0, 0.0), Point::new(2.0, 0.0)));
        set.insert(Segment::new(Point::new(2.0, 0.0), Point::new(0.5, 1.0)));
        set
    };

    let result_set: HashSet<_> = edges.into_iter().collect();
    assert_eq!(result_set, expected_edges);
}

#[rstest]
fn test_edge_triple_occurrence() {
    // Create three triangles where one edge appears three times
    let polygons = vec![
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(0.5, 1.0),
        ],
        vec![
            Point::new(1.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(0.5, 1.0),
        ],
    ];

    let edges = calc_dedup_edges(&polygons);

    // Any edge that appears three times should be present in the final result once
    // Count edges connecting to Point(0.5, 1.0)
    let center_point = Point::new(0.5, 1.0);
    let edges_to_center = edges
        .iter()
        .filter(|segment| segment.top == center_point || segment.bottom == center_point)
        .count();

    assert_eq!(edges_to_center, 4);
}
