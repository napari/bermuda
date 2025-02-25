use rstest::rstest;

use std::collections::HashSet;
use triangulation::face_triangulation::sweeping_line_triangulation;
use triangulation::point::{calc_dedup_edges, Point};

const DIAMOND: [Point; 4] = [
    Point::new(1.0, 0.0),
    Point::new(2.0, 1.0),
    Point::new(1.0, 2.0),
    Point::new(0.0, 1.0),
];

#[rstest]
fn test_diamond() {
    let (triangles, points) =
        sweeping_line_triangulation(calc_dedup_edges(&vec![DIAMOND.to_vec()]));
    assert_eq!(triangles.len(), 2);
    assert_eq!(points.len(), 4);
    assert_eq!(
        points.into_iter().collect::<HashSet<_>>(),
        DIAMOND.iter().cloned().collect::<HashSet<_>>()
    );
}
