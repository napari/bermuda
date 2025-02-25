//! This library provides computational algorithms for triangulation.
//!
//! These algorithms are designed for performance when working with polygons.

pub mod face_triangulation;
pub mod intersection;
pub mod monotone_polygon;
pub mod path_triangulation;
pub mod point;

pub use crate::path_triangulation::triangulate_path_edge;
pub use crate::path_triangulation::PathTriangulation;
pub use crate::point::Point;
