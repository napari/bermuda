use rstest::rstest;
use triangulation::intersection;
use triangulation::point;

#[rstest]
#[case(0.0, 0.0, 0.5, 0.5, 1.0, 1.0, true)]
#[case(0.0, 0.0, 0.0, 0.5, 0.0, 1.0, true)]
#[case(0.0, 0.0, 0.5, 0.0, 1.0, 0.0, true)]
#[case(0.0, 0.0, 1.0, 1.0, 0.5, 0.5, false)]
#[case(0.0, 0.0, 0.0, 1.0, 0.0, 0.5, false)]
#[case(0.0, 0.0, 1.0, 0.0, 0.5, 0.0, false)]
fn test_on_segment_if_collinear(
    #[case] p_x: f32,
    #[case] p_y: f32,
    #[case] q_x: f32,
    #[case] q_y: f32,
    #[case] r_x: f32,
    #[case] r_y: f32,
    #[case] expected: bool,
) {
    let p = point::Point::new(p_x, p_y);
    let q = point::Point::new(q_x, q_y);
    let r = point::Point::new(r_x, r_y);
    assert_eq!(intersection::on_segment_if_collinear(p, q, r), expected);
}
