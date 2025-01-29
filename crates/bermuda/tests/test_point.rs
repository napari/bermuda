use rstest::rstest;
use triangulation::point::{Point, Segment, Vector};

#[rstest]
fn test_segment_order() {
    let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(0.0, 0.0));
    assert_eq!(s1, s2);
}

#[rstest]
fn test_vector_add() {
    assert_eq!(
        Point::new(1.0, 0.0) + Vector::new(1.0, 1.0),
        Point::new(2.0, 1.0)
    );
}
