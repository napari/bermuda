use numpy::{PyArray, PyArray2, PyArrayMethods, PyReadonlyArray2};
use pyo3::prelude::*;

use triangulation::point::Triangle;
use triangulation::{
    split_polygons_on_repeated_edges, sweeping_line_triangulation,
    triangulate_path_edge as triangulate_path_edge_rust, triangulate_paths_edge, PathTriangulation,
    Point,
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
    triangulation_to_edge_triangulate(py, &result)
}

fn triangulation_to_edge_triangulate(
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

fn triangulation_to_face_triangulate(
    py: Python<'_>,
    triangles: &[Triangle],
    points: &[Point],
) -> PyFaceTriangulation {
    let triangle_data: Vec<u32> = triangles
        .iter()
        .flat_map(|t| [t.x as u32, t.y as u32, t.z as u32])
        .collect();

    let triangle_array = if !triangles.is_empty() {
        PyArray::from_vec(py, triangle_data).reshape([triangles.len(), 3])?
    } else {
        PyArray2::<u32>::zeros(py, [0, 3], false)
    };
    let flat_points: Vec<f32> = points.iter().flat_map(|p| [p.x, p.y]).collect();
    Ok((
        triangle_array.into(),
        PyArray::from_vec(py, flat_points)
            .reshape([points.len(), 2])?
            .into(),
    ))
}

#[pyfunction]
#[pyo3(signature = (polygons))]
fn triangulate_polygons_with_edge(
    py: Python<'_>,
    polygons: Vec<PyReadonlyArray2<'_, f32>>,
) -> PyPolygonTriangulation {
    // Convert the numpy array into a rust compatible representations which is a vector of points.
    let polygons_: Vec<Vec<Point>> = polygons
        .into_iter()
        .map(|polygon| {
            polygon
                .as_array()
                .rows()
                .into_iter()
                .map(|row| Point {
                    x: row[0],
                    y: row[1],
                })
                .collect()
        })
        .collect();

    let (new_polygons, segments) = split_polygons_on_repeated_edges(&polygons_);
    let (face_triangles, face_points) = sweeping_line_triangulation(segments);
    let path_triangulation = triangulate_paths_edge(&new_polygons, true, 3.0, false);
    Ok((
        triangulation_to_face_triangulate(py, &face_triangles, &face_points)?,
        triangulation_to_edge_triangulate(py, &path_triangulation)?,
    ))
}

#[pymodule]
fn _bermuda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangulate_path_edge, m)?)?;
    m.add_function(wrap_pyfunction!(triangulate_polygons_with_edge, m)?)?;
    Ok(())
}
