use rstest::rstest;

use triangulation::path_triangulation::triangulate_path_edge;
use triangulation::point::Point;

#[rstest]
fn test_path_non_convex_polygon() {
    let polygon = vec![
        Point::new(10.97627008, 14.30378733),
        Point::new(12.05526752, 10.89766366),
        Point::new(8.47309599, 12.91788226),
        Point::new(8.75174423, 17.83546002),
        Point::new(19.27325521, 7.66883038),
        Point::new(15.83450076, 10.5778984),
    ];
    let result = triangulate_path_edge(&polygon, true, 3.0, false);
    assert_eq!(result.centers.len(), 16);
    assert_eq!(result.triangles.len(), 14);
}
