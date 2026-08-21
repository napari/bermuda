//! Regression test for the failure reported in napari/bermuda#194.
//!
//! This mirrors the Python `test_hole_triangulation_194` test, so the failing
//! code path can be run (and debugged) without going through pyo3/numpy.

use rstest::rstest;
use triangulation::{
    is_convex, split_polygons_on_repeated_edges, sweeping_line_triangulation,
    triangulate_convex_polygon, Point, Triangle,
};

/// The same vertices the Python test loads from
/// `tests/data/create_holes_triangulation_failure.txt`.
const POLYGON_DATA: &str =
    include_str!("../../../tests/data/create_holes_triangulation_failure.txt");

/// Parse whitespace separated `x y` pairs the way
/// `np.loadtxt(..., dtype=np.float32)` does: read as f64, then narrow to f32.
fn parse_points(data: &str) -> Vec<Point> {
    data.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut coordinates = line
                .split_whitespace()
                .map(|value| value.parse::<f64>().expect("invalid coordinate") as f32);
            let x = coordinates.next().expect("missing x coordinate");
            let y = coordinates.next().expect("missing y coordinate");
            assert!(
                coordinates.next().is_none(),
                "expected exactly two coordinates per line"
            );
            Point::new(x, y)
        })
        .collect()
}

/// The single polygon branch of `bermuda::triangulate_polygons_face`, without
/// the Python bindings. Consecutive duplicate points are dropped by
/// `numpy_polygons_to_rust_polygons`, so the caller does that beforehand.
fn triangulate_polygon_face(polygon: &[Point]) -> (Vec<Triangle>, Vec<Point>) {
    if polygon.len() < 3 {
        return (vec![Triangle::new(0, 0, 0)], polygon.to_vec());
    }
    if polygon.len() == 3 {
        return (vec![Triangle::new(0, 1, 2)], polygon.to_vec());
    }
    if is_convex(polygon) {
        return (triangulate_convex_polygon(polygon), polygon.to_vec());
    }
    let (_new_polygons, segments) = split_polygons_on_repeated_edges(&[polygon.to_vec()]);
    sweeping_line_triangulation(segments)
}

#[rstest]
fn test_hole_triangulation_194() {
    let mut polygon = parse_points(POLYGON_DATA);
    assert_eq!(polygon.len(), 570);
    assert!(polygon.iter().all(|p| p.x.is_finite() && p.y.is_finite()));

    polygon.dedup();

    let (triangles, points) = triangulate_polygon_face(&polygon);

    assert!(!points.is_empty());
    assert!(!triangles.is_empty());
    let point_count = points.len();
    assert!(triangles
        .iter()
        .all(|t| t.x < point_count && t.y < point_count && t.z < point_count));
}
