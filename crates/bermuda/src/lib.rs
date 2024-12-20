use numpy::{IntoPyArray, PyArray2, PyArrayMethods, PyReadonlyArray2, ToPyArray};
use pyo3::prelude::*;

use triangulation::{
    triangulate_path_edge as triangulate_path_edge_rust, PathTriangulation, Point,
};

/// Determines the triangulation of a path in 2D
///
/// Parameters
/// ----------
///path : np.ndarray
///     Nx2 array of central coordinates of path to be triangulated
/// closed : bool
///     Bool which determines if the path is closed or not
/// limit : float
///     Miter limit which determines when to switch from a miter join to a
///     bevel join
/// bevel : bool
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
fn triangulate_path_edge<'py>(
    py: Python<'_>,
    path: PyReadonlyArray2<'_, f32>,
    closed: bool,
    limit: f32,
    bevel: bool,
) -> PyResult<(Py<PyArray2<f32>>, Py<PyArray2<f32>>, Py<PyArray2<u32>>)> {
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
    let result = triangulate_path_edge_rust(&path_, closed, limit, bevel);
    let triangle_data: Vec<Vec<u32>> = result
        .triangles
        .iter()
        .map(|t| vec![t.x as u32, t.y as u32, t.z as u32])
        .collect();
    
    // Convert back to numpy array ((M-2)x3) if triangles is not empty, otherwise create empty array (0x3).
    let triangle_array = if !result.triangles.is_empty() {
        PyArray2::<u32>::from_vec2(py, &triangle_data)?
    } else {
        PyArray2::<u32>::zeros(py, [0, 3], false)
    };

    let centers: Vec<Vec<f32>> = result.centers.iter().map(|p| vec![p.x, p.y]).collect();
    let offsets: Vec<Vec<f32>> = result.offsets.iter().map(|p| vec![p.x, p.y]).collect();

    Ok((
        PyArray2::<f32>::from_vec2(py, &centers)?.into(),
        PyArray2::<f32>::from_vec2(py, &offsets)?.into(),
        triangle_array.into(),
    ))
}

#[pymodule]
fn _bermuda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangulate_path_edge, m)?)?;
    Ok(())
}
