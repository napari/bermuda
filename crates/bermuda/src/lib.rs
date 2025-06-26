#![allow(clippy::useless_conversion)]

use numpy::{PyArray, PyArray2, PyArrayMethods, PyReadonlyArray2};
use pyo3::prelude::*;

use triangulation::{
    is_convex, split_polygons_on_repeated_edges as split_polygons_on_repeated_edges_rust,
    sweeping_line_triangulation, triangulate_convex_polygon,
    triangulate_path_edge as triangulate_path_edge_rust, triangulate_paths_edge, PathTriangulation,
    Point, Triangle,
};

type EdgeTriangulation = (Py<PyArray2<f32>>, Py<PyArray2<f32>>, Py<PyArray2<u32>>);
type FaceTriangulation = (Py<PyArray2<u32>>, Py<PyArray2<f32>>);
type PyEdgeTriangulation = PyResult<EdgeTriangulation>;
type PyFaceTriangulation = PyResult<FaceTriangulation>;
type PyPolygonTriangulation = PyResult<(FaceTriangulation, EdgeTriangulation)>;

/// Determines the triangulation of a path in 2D
///
/// Parameters
/// ----------
///path : np.ndarray
///     Nx2 array of central coordinates of path to be triangulated
/// closed : bool, optional (default=False)
///     Bool which determines if the path is closed or not
/// limit : float, optional (default=3.0)
///     Miter limit which determines when to switch from a miter join to a
///     bevel join
/// bevel : bool, optional (default=False)
///     Bool which if True causes a bevel join to always be used. If False
///     a bevel join will only be used when the miter limit is exceeded
//
/// Returns
/// -------
/// centers : np.ndarray
///     Mx2 array central coordinates of path triangles.
/// offsets : np.ndarray
///     Mx2 array of the offsets to the central coordinates that need to
///     be scaled by the line width and then added to the centers to
///     generate the actual vertices of the triangulation
/// triangles : np.ndarray
///     (M-2)x3 array of the indices of the vertices that will form the
///     triangles of the triangulation
#[pyfunction]
#[pyo3(signature = (path, closed=false, limit=3.0, bevel=false))]
fn triangulate_path_edge(
    py: Python<'_>,
    path: PyReadonlyArray2<'_, f32>,
    closed: Option<bool>,
    limit: Option<f32>,
    bevel: Option<bool>,
) -> PyEdgeTriangulation {
    // Convert the numpy array into a rust compatible representations which is a vector of points.
    let path_: Vec<Point> = path
        .as_array()
        .rows()
        .into_iter()
        .map(|row| Point {
            x: row[0],
            y: row[1],
        })
        .collect();

    // Call the re-exported Rust function directly
    let result = triangulate_path_edge_rust(
        &path_,
        closed.unwrap_or(false),
        limit.unwrap_or(3.0),
        bevel.unwrap_or(false),
    );
    path_triangulation_to_numpy_arrays(py, &result)
}

/// Convert internal representation of path triangulation into numpy arrays
fn path_triangulation_to_numpy_arrays(
    py: Python<'_>,
    data: &PathTriangulation,
) -> PyEdgeTriangulation {
    let triangle_data: Vec<u32> = data
        .triangles
        .iter()
        .flat_map(|t| [t.x as u32, t.y as u32, t.z as u32])
        .collect();

    let triangle_array = if !data.triangles.is_empty() {
        PyArray::from_vec(py, triangle_data).reshape([data.triangles.len(), 3])?
    } else {
        PyArray2::<u32>::zeros(py, [0, 3], false)
    };

    let flat_centers: Vec<f32> = data.centers.iter().flat_map(|p| [p.x, p.y]).collect();
    let flat_offsets: Vec<f32> = data.offsets.iter().flat_map(|v| [v.x, v.y]).collect();

    Ok((
        PyArray::from_vec(py, flat_centers)
            .reshape([data.centers.len(), 2])?
            .into(),
        PyArray::from_vec(py, flat_offsets)
            .reshape([data.offsets.len(), 2])?
            .into(),
        triangle_array.into(),
    ))
}

fn triangles_to_numpy_array(py: Python<'_>, triangles: &[Triangle]) -> Py<PyArray2<u32>> {
    let triangle_data: Vec<u32> = triangles
        .iter()
        .flat_map(|t| [t.x as u32, t.y as u32, t.z as u32])
        .collect();
    if !triangles.is_empty() {
        PyArray::from_vec(py, triangle_data)
            .reshape([triangles.len(), 3])
            .unwrap()
            .into()
    } else {
        PyArray2::<u32>::zeros(py, [0, 3], false).into()
    }
}

/// Convert internal representation of face triangulation into numpy arrays
fn face_triangulation_to_numpy_arrays(
    py: Python<'_>,
    triangles: &[Triangle],
    points: &[Point],
) -> PyFaceTriangulation {
    let flat_points: Vec<f32> = points.iter().flat_map(|p| [p.x, p.y]).collect();
    Ok((
        triangles_to_numpy_array(py, triangles),
        PyArray::from_vec(py, flat_points)
            .reshape([points.len(), 2])?
            .into(),
    ))
}

fn numpy_polygons_to_rust_polygons(polygons: Vec<PyReadonlyArray2<'_, f32>>) -> Vec<Vec<Point>> {
    let polygons_: Vec<Vec<Point>> = polygons
        .into_iter()
        .map(|polygon| {
            let mut points: Vec<Point> = polygon
                .as_array()
                .rows()
                .into_iter()
                .map(|row| Point {
                    x: row[0],
                    y: row[1],
                })
                .collect();
            points.dedup();
            points
        })
        .collect();
    polygons_
}

/// Projects 3D polygons onto a 2D plane by dropping the axis along which all points are collinear.
///
/// Determines which coordinate axis is constant across all points in all polygons, then projects each polygon onto the remaining two axes. Consecutive duplicate points are removed from each projected polygon.
///
/// # Returns
/// A tuple containing:
/// - The projected 2D polygons as vectors of `Point`.
/// - The index of the dropped axis (0 for x, 1 for y, 2 for z).
/// - The constant value along the dropped axis.
///
/// If the input polygons are not coplanar (i.e., no axis is constant across all points), returns empty polygons, zero for the dropped axis, and zero for the dropped value.
fn numpy_polygons_to_rust_polygons_3d(
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> (Vec<Vec<Point>>, usize, f32) {
    let mut is_collinearity_axis = [true; 3];
    let first_coordinates = [
        polygons[0].as_array().row(0)[0],
        polygons[0].as_array().row(0)[1],
        polygons[0].as_array().row(0)[2],
    ];

    for polygon in &polygons {
        let polygon_ = polygon.as_array();
        for point in polygon_.rows() {
            is_collinearity_axis[0] = is_collinearity_axis[0] && point[0] == first_coordinates[0];
            is_collinearity_axis[1] = is_collinearity_axis[1] && point[1] == first_coordinates[1];
            is_collinearity_axis[2] = is_collinearity_axis[2] && point[2] == first_coordinates[2];
        }
    }

    let count_false = is_collinearity_axis.iter().filter(|&&x| !x).count();

    if count_false != 2 {
        // Either points are collinear against one of the axis or there
        // is no hyperplane defined by axis that contains all points.
        return (Vec::new(), 0, 0.0);
    }

    let false_positions: Vec<usize> = is_collinearity_axis
        .iter()
        .enumerate()
        .filter(|(_, &value)| !value)
        .map(|(index, _)| index)
        .collect();

    let drop_axis = is_collinearity_axis.iter().position(|&x| x).unwrap();
    let drop_value = first_coordinates[drop_axis];

    // Now false_positions contains [0, 2]
    let pos1 = false_positions[0]; // 0
    let pos2 = false_positions[1]; // 2

    let polygons_: Vec<Vec<Point>> = polygons
        .into_iter()
        .map(|polygon| {
            let mut points: Vec<Point> = polygon
                .as_array()
                .rows()
                .into_iter()
                .map(|row| Point {
                    x: row[pos1],
                    y: row[pos2],
                })
                .collect();
            points.dedup();
            points
        })
        .collect();
    (polygons_, drop_axis, drop_value)
}

fn face_triangulate_single_polygon(polygon: &[Point]) -> Option<Vec<Triangle>> {
    if polygon.len() < 3 {
        return Some(vec![Triangle::new(0, 0, 0)]);
    }
    if polygon.len() == 3 {
        return Some(vec![Triangle::new(0, 1, 2)]);
    }
    if is_convex(polygon) {
        return Some(triangulate_convex_polygon(polygon));
    }
    None
}

/// Triangulates multiple polygons and generates both face and edge triangulations
///
/// This function performs two types of triangulation:
/// 1. Face triangulation of the polygon interiors
/// 2. Edge triangulation for the polygon boundaries
///
/// Parameters
/// ----------
/// polygons : List[numpy.ndarray]
///     List of Nx2 arrays where each array contains the vertices of a polygon
///     as (x, y) coordinates. Each polygon should be defined in counter-clockwise order.
///
/// Returns
/// -------
/// tuple
///     A tuple containing two elements:
///     
///     1. Face triangulation (tuple):
///         - triangles : numpy.ndarray
///             Mx3 array of vertex indices forming triangles
///         - points : numpy.ndarray
///             Px2 array of vertex coordinates
///     
///     2. Edge triangulation (tuple):
///         - centers : numpy.ndarray
///             Qx2 array of central coordinates of edge triangles
///         - offsets : numpy.ndarray
///             Qx2 array of offset vectors for edge vertices
///         - triangles : numpy.ndarray
///             Rx3 array of vertex indices for edge triangles
///
/// Notes
/// -----
/// The function first processes any self-intersecting edges and repeated vertices,
/// then performs face triangulation using a sweeping line algorithm, and finally
/// generates edge triangulation for the polygon boundaries.
///
/// The edge triangulation uses default parameters:
/// - closed = true (treats polygons as closed)
/// - miter_limit = 3.0
/// - bevel = false (uses miter joins by default)
#[pyfunction]
#[pyo3(signature = (polygons))]
fn triangulate_polygons_with_edge(
    py: Python<'_>,
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> PyPolygonTriangulation {
    // Convert the numpy array into a rust compatible representation which is a vector of points.
    let polygons_ = numpy_polygons_to_rust_polygons(polygons);
    if polygons_.len() == 1 {
        if let Some(result) = face_triangulate_single_polygon(&polygons_[0]) {
            let path_triangulation = triangulate_paths_edge(&polygons_, true, 3.0, false);
            return Ok((
                face_triangulation_to_numpy_arrays(py, &result, &polygons_[0])?,
                path_triangulation_to_numpy_arrays(py, &path_triangulation)?,
            ));
        }
    }

    let (new_polygons, segments) = split_polygons_on_repeated_edges_rust(&polygons_);
    let (face_triangles, face_points) = sweeping_line_triangulation(segments);
    let path_triangulation = triangulate_paths_edge(&new_polygons, true, 3.0, false);
    Ok((
        face_triangulation_to_numpy_arrays(py, &face_triangles, &face_points)?,
        path_triangulation_to_numpy_arrays(py, &path_triangulation)?,
    ))
}

/// Performs face triangulation of multiple polygons
///
/// Parameters
/// ----------
/// polygons : List[numpy.ndarray]
///     List of Nx2 arrays where each array contains the vertices of a polygon
///     as (x, y) coordinates. Each polygon should be defined in counter-clockwise order.
///
/// Returns
/// -------
/// tuple
///     A tuple containing two elements:
///     - triangles : numpy.ndarray
///         Mx3 array of vertex indices that form the triangulation
///     - points : numpy.ndarray
///         Px2 array of vertex coordinates used in the triangulation
///
/// Notes
/// -----
/// The function processes the input polygons by:
/// 1. Converting the input NumPy arrays to Rust-compatible polygon representation
/// 2. Handling self-intersecting edges and repeated vertices
/// 3. Performing face triangulation using a sweeping line algorithm
///
/// The function returns only the face triangulation without edge triangulation,
/// making it suitable for cases where only the interior triangulation is needed.
#[pyfunction]
#[pyo3(signature = (polygons))]
fn triangulate_polygons_face(
    py: Python<'_>,
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> PyFaceTriangulation {
    // Convert the numpy array into a rust compatible representation which is a vector of points.
    let polygons_ = numpy_polygons_to_rust_polygons(polygons);

    if polygons_.len() == 1 {
        if let Some(result) = face_triangulate_single_polygon(&polygons_[0]) {
            return face_triangulation_to_numpy_arrays(py, &result, &polygons_[0]);
        }
    }

    let (_new_polygons, segments) = split_polygons_on_repeated_edges_rust(&polygons_);
    let (face_triangles, face_points) = sweeping_line_triangulation(segments);
    face_triangulation_to_numpy_arrays(py, &face_triangles, &face_points)
}

fn convert_rust_polygons_to_py_arrays(
    py: Python<'_>,
    polygons: Vec<Vec<Point>>,
) -> PyResult<Vec<Py<PyArray2<f32>>>> {
    let mut py_arrays = Vec::with_capacity(polygons.len());

    for polygon in polygons {
        let num_points = polygon.len();
        let flat_points: Vec<f32> = polygon.iter().flat_map(|p| [p.x, p.y]).collect();

        // Create a PyArray from the flat vector
        let array = PyArray::from_vec(py, flat_points);

        // Reshape the array to [num_points, 2]
        // The reshape operation returns a PyResult, so we use '?' to propagate errors
        let reshaped_array = array.reshape([num_points, 2])?;

        // Convert the borrowed array (&PyArray2) to an owned one (Py<PyArray2>)
        py_arrays.push(reshaped_array.into());
    }

    Ok(py_arrays)
}

/// Splits polygons that have repeated edges into separate polygons
///
/// Parameters
/// ----------
/// polygons : List[numpy.ndarray]
///     List of Nx2 arrays where each array contains the vertices of a polygon
///     as (x, y) coordinates.
///
/// Returns
/// -------
/// List[numpy.ndarray]
///     A list of Mx2 arrays where each array represents a polygon after splitting
///     at repeated edges. Consecutive duplicate points are automatically removed.
#[pyfunction]
#[pyo3(signature = (polygons))]
fn split_polygons_on_repeated_edges(
    py: Python<'_>,
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> PyResult<Vec<Py<PyArray2<f32>>>> {
    let polygons_ = numpy_polygons_to_rust_polygons(polygons);
    let (new_polygons, _segments) = split_polygons_on_repeated_edges_rust(&polygons_);
    convert_rust_polygons_to_py_arrays(py, new_polygons)
}

#[pyfunction]
#[pyo3(signature = (polygons))]
fn triangulate_polygons_face_3d(
    py: Python<'_>,
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> PyFaceTriangulation {
    // Convert the numpy array into a rust compatible representation which is a vector of points.
    let (polygons_, drop_axis, drop_value) = numpy_polygons_to_rust_polygons_3d(polygons);
    let (face_triangles, face_points) = if polygons_.len() == 1 {
        if let Some(result) = face_triangulate_single_polygon(&polygons_[0]) {
            (result, polygons_[0].clone())
        } else {
            sweeping_line_triangulation(split_polygons_on_repeated_edges_rust(&polygons_).1)
        }
    } else {
        sweeping_line_triangulation(split_polygons_on_repeated_edges_rust(&polygons_).1)
    };

    let triangles = triangles_to_numpy_array(py, &face_triangles);

    let flat_points: Vec<f32> = if drop_axis == 0 {
        face_points
            .iter()
            .flat_map(|p| [drop_value, p.x, p.y])
            .collect()
    } else if drop_axis == 1 {
        face_points
            .iter()
            .flat_map(|p| [p.x, drop_value, p.y])
            .collect()
    } else {
        face_points
            .iter()
            .flat_map(|p| [p.x, p.y, drop_value])
            .collect()
    };

    Ok((
        triangles,
        PyArray::from_vec(py, flat_points)
            .reshape([face_points.len(), 3])?
            .into(),
    ))
}

#[pymodule]
fn _bermuda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangulate_path_edge, m)?)?;
    m.add_function(wrap_pyfunction!(triangulate_polygons_with_edge, m)?)?;
    m.add_function(wrap_pyfunction!(triangulate_polygons_face, m)?)?;
    m.add_function(wrap_pyfunction!(triangulate_polygons_face_3d, m)?)?;
    m.add_function(wrap_pyfunction!(split_polygons_on_repeated_edges, m)?)?;
    Ok(())
}
