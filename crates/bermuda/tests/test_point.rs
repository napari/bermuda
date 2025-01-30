use rstest::rstest;
use triangulation::point::{orientation, Orientation, Point, Segment, Vector};

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
