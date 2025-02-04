use rstest::rstest;
use triangulation::monotone_polygon::{triangulate_monotone_polygon, MonotonePolygon};
use triangulation::point::{Point, PointTriangle};

#[rstest]
fn test_monotone_polygon_simple() {
    let top = Point::new(0.0, 10.0);
    let left = Point::new(-1.0, 7.0);
    let right = Point::new(1.0, 5.0);
    let bottom = Point::new(0.0, 0.0);
    let mut poly = MonotonePolygon::new_top(top);
    assert!(!poly.finished());
    poly.left.push(left);
    poly.right.push(right);
    assert!(!poly.finished());
    poly.bottom = Option::from(bottom);
    assert!(poly.finished());
    let result = triangulate_monotone_polygon(&poly);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], PointTriangle::new(right, top, left));
    assert_eq!(result[1], PointTriangle::new(bottom, left, right));
}

#[rstest]
#[case::diamond(
    MonotonePolygon::new(Point::new(1.0, 2.0), Point::new(1.0, 0.0), vec![Point::new(0.0, 1.0)], vec![Point::new(2.0, 1.0)]),
    vec![PointTriangle::new(Point::new(2.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 1.0)), PointTriangle::new(Point::new(1.0, 0.0), Point::new(2.0, 1.0), Point::new(0.0, 1.0))]
)]
#[case::wide_diamond(
    MonotonePolygon::new(Point::new(5.0, 2.0), Point::new(5.0, 0.0), vec![Point::new(0.0, 1.0)], vec![Point::new(2.0, 1.0)]),
    vec![PointTriangle::new(Point::new(2.0, 1.0), Point::new(5.0, 2.0), Point::new(0.0, 1.0)), PointTriangle::new(Point::new(5.0, 0.0), Point::new(2.0, 1.0), Point::new(0.0, 1.0))]
)]
#[case::double_diamond(
    MonotonePolygon::new(Point::new(1.0, 3.0), Point::new(1.0, 0.0),
        vec![Point::new(0.0, 2.0), Point::new(0.0, 1.0)],
        vec![Point::new(2.0, 2.0), Point::new(2.0, 1.0)]),
    vec![
        PointTriangle::new(Point::new(2.0, 2.0), Point::new(1.0, 3.0), Point::new(0.0, 2.0)),
        PointTriangle::new(Point::new(2.0, 1.0), Point::new(2.0, 2.0), Point::new(0.0, 2.0)),
        PointTriangle::new(Point::new(2.0, 1.0), Point::new(0.0, 2.0), Point::new(0.0, 1.0)),
        PointTriangle::new(Point::new(1.0, 0.0), Point::new(2.0, 1.0), Point::new(0.0, 1.0))
    ]
)]
#[case::zigzag_right(
    MonotonePolygon::new(Point::new(0.0, 4.0), Point::new(0.5, 0.0),
        vec![],
        vec![Point::new(2.0, 3.0), Point::new(3.0, 2.0), Point::new(2.0, 1.0)]),
    vec![
        PointTriangle::new(Point::new(3.0, 2.0), Point::new(2.0, 3.0), Point::new(0.0, 4.0)),
        PointTriangle::new(Point::new(2.0, 1.0), Point::new(3.0, 2.0), Point::new(0.0, 4.0)),
        PointTriangle::new(Point::new(2.0, 1.0), Point::new(0.0, 4.0), Point::new(0.5, 0.0))
    ]
)]
#[case::zigzag_right_shallow(
    MonotonePolygon::new(Point::new(0.0, 4.0), Point::new(0.0, 0.0),
        vec![],
        vec![Point::new(2.0, 3.0), Point::new(1.0, 2.0), Point::new(2.0, 1.0)]),
    vec![
        PointTriangle::new(Point::new(1.0, 2.0), Point::new(2.0, 3.0), Point::new(0.0, 4.0)),
        PointTriangle::new(Point::new(1.0, 2.0), Point::new(0.0, 4.0), Point::new(0.0, 0.0)),
        PointTriangle::new(Point::new(2.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 0.0))
    ]
)]
fn test_monotone_polygon(#[case] poly: MonotonePolygon, #[case] expected: Vec<PointTriangle>) {
    let result = triangulate_monotone_polygon(&poly);
    assert_eq!(result, expected);
}
