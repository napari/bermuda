pub mod path_triangulation;
pub mod point;

pub use crate::path_triangulation::triangulation::triangulation::triangulate_path_edge;
pub use crate::path_triangulation::triangulation::triangulation::PathTriangulation;
pub use crate::point::triangulation::point::Point;
