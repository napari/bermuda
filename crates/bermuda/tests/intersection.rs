use rstest::rstest;
use triangulation::intersection;
use triangulation::point::{Point, Segment};

#[rstest]
#[case(Point::new(0.0, 0.0), Point::new(0.5, 0.5), Point::new(1.0, 1.0), true)]
#[case(Point::new(0.0, 0.0), Point::new(0.0, 0.5), Point::new(0.0, 1.0), true)]
#[case(Point::new(0.0, 0.0), Point::new(0.5, 0.0), Point::new(1.0, 0.0), true)]
#[case(Point::new_i(0, 0), Point::new_i(1, 1), Point::new(0.5, 0.5), false)]
#[case(Point::new_i(0, 0), Point::new_i(0, 1), Point::new(0.0, 0.5), false)]
#[case(Point::new_i(0, 0), Point::new_i(1, 0), Point::new(0.5, 0.0), false)]
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
    assert_ne!(
        intersection::do_intersect(
            &Segment::new(Point::new_i(0, -1), Point::new_i(0, 1)),
            &Segment::new(Point::new_i(1, -2), Point::new_i(1, 1))
        ),
        true
    )
}

#[rstest]
#[case(Segment::new_i((0, 0), (2, 2)), Segment::new_i((2, 0), (0, 2)), Point::new_i(1, 1))]
#[case(Segment::new_i((0, 0), (1, 0)), Segment::new_i((0, 1), (0, 0)), Point::new_i(0, 0))]
#[case(Segment::new_i((0, 0), (2, 0)), Segment::new_i((1, 0), (1, 2)), Point::new_i(1, 0))]
fn test_find_intersection_point(#[case] s1: Segment, #[case] s2: Segment, #[case] expected: Point) {
    assert_eq!(intersection::find_intersection(&s1, &s2)[0], expected);
    assert_eq!(intersection::find_intersection(&s2, &s1)[0], expected);
}

#[rstest]
fn test_find_intersection_collinear_segments() {
    assert_eq!(
        intersection::find_intersection(
            &Segment::new_i((0, 0), (2, 0)),
            &Segment::new_i((1, 0), (3, 0))
        )[0],
        Point::new_i(1, 0)
    );
    assert_eq!(
        intersection::find_intersection(
            &Segment::new_i((0, 0), (2, 0)),
            &Segment::new_i((1, 0), (3, 0))
        )[1],
        Point::new_i(2, 0)
    );
}
