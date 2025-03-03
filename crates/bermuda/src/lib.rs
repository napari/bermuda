#![allow(clippy::useless_conversion)]
use numpy::{PyArray, PyArray2, PyArrayMethods, PyReadonlyArray2};
use pyo3::prelude::*;

use triangulation::{triangulate_path_edge as triangulate_path_edge_rust, Point};

type EdgeTriangulation = PyResult<(Py<PyArray2<f32>>, Py<PyArray2<f32>>, Py<PyArray2<u32>>)>;

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
) -> EdgeTriangulation {
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
    let triangle_data: Vec<u32> = result
        .triangles
        .iter()
        .flat_map(|t| [t.x as u32, t.y as u32, t.z as u32])
        .collect();

    // Convert back to numpy array ((M-2)x3) if triangles is not empty, otherwise create empty array (0x3).
    let triangle_array = if !result.triangles.is_empty() {
        PyArray::from_vec(py, triangle_data).reshape([result.triangles.len(), 3])?
    } else {
        PyArray2::<u32>::zeros(py, [0, 3], false)
    };

    let flat_centers: Vec<f32> = result.centers.iter().flat_map(|p| [p.x, p.y]).collect();
    let flat_offsets: Vec<f32> = result.offsets.iter().flat_map(|v| [v.x, v.y]).collect();

    Ok((
        PyArray::from_vec(py, flat_centers)
            .reshape([result.centers.len(), 2])?
            .into(),
        PyArray::from_vec(py, flat_offsets)
            .reshape([result.offsets.len(), 2])?
            .into(),
        triangle_array.into(),
    ))
}

#[pymodule]
fn _bermuda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangulate_path_edge, m)?)?;
    Ok(())
}
