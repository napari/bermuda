use rstest::rstest;
use triangulation::monotone_polygon::{triangulate_monotone_polygon, MonotonePolygon};
use triangulation::point::{Point, PointTriangle};

#[rstest]
fn test_monotone_polygon_simple() {
    let top = Point::new(0.0, 10.0);
    let left = Point::new(-1.0, 7.0);
    let right = Point::new(1.0, 5.0);
    let bottom = Point::new(0.0, 0.0);
    let mut poly = MonotonePolygon::new(top);
    assert!(!poly.finished());
    poly.left.push(left);
    poly.right.push(right);
    assert!(!poly.finished());
    poly.bottom = Option::from(bottom);
    assert!(poly.finished());
    let result = triangulate_monotone_polygon(&poly);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], PointTriangle::new(top, left, right));
    assert_eq!(result[1], PointTriangle::new(left, right, bottom));
}
