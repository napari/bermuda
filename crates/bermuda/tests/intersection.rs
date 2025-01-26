use rstest::rstest;
use triangulation::intersection;
use triangulation::point::Point;

#[rstest]
#[case(Point::new(0.0, 0.0), Point::new(0.5, 0.5), Point::new(1.0, 1.0), true)]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 0.5), Point::new(0.0, 1.0), true)]
#[case(Point::new(0.0, 0.0), Point::new(0.5, 0.0), Point::new(1.0, 0.0), true)]
#[case(
    Point::new(0.0, 0.0),
    Point::new(1.0, 1.0),
    Point::new(0.5, 0.5),
    false
)]
#[case(
    Point::new(0.0, 0.0),
    Point::new(0.0, 1.0),
    Point::new(0.0, 0.5),
    false
)]
#[case(
    Point::new(0.0, 0.0),
    Point::new(1.0, 0.0),
    Point::new(0.5, 0.0),
    false
)]
fn test_on_segment_if_collinear(
    #[case] p: Point,
    #[case] q: Point,
    #[case] r: Point,
    #[case] expected: bool,
) {
    assert_eq!(intersection::on_segment_if_collinear(p, q, r), expected);
}

#[rstest]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(0.0, 2.0), 0)]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 2.0), Point::new(0.0, 1.0), 0)]
#[case(Point::new(0.0, 2.0), Point::new(0.0, 0.0), Point::new(0.0, 1.0), 0)]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 2.0), 1)]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(-1.0, 2.0), 2)]
fn test_orientation(#[case] p: Point, #[case] q: Point, #[case] r: Point, #[case] expected: i32) {
    assert_eq!(intersection::orientation(p, q, r), expected);
}
