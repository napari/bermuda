use rstest::rstest;
use triangulation::point::{Point, Segment, Vector};

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
