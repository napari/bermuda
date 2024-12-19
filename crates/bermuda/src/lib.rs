use numpy::{IntoPyArray, PyArray2, PyArrayMethods, PyReadonlyArray2, ToPyArray};
use pyo3::prelude::*;

use triangulation::{
    triangulate_path_edge as triangulate_path_edge_rust, PathTriangulation, Point,
};

#[pyfunction]
fn triangulate_path_edge<'py>(
    py: Python<'_>,
    x: PyReadonlyArray2<'_, f32>,
    closed: bool,
    bevel: bool,
) -> PyResult<(Py<PyArray2<f32>>, Py<PyArray2<f32>>, Py<PyArray2<u32>>)> {
    let path: Vec<Point> = x
        .as_array()
        .rows()
        .into_iter()
        .map(|row| Point {
            x: row[0],
            y: row[1],
        })
        .collect();

    // Call the re-exported Rust function directly
    let result = triangulate_path_edge_rust(&path, closed, 0.1, bevel);
    let triangle_data: Vec<Vec<u32>> = result
        .triangles
        .iter()
        .map(|t| vec![t.x as u32, t.y as u32, t.z as u32])
        .collect();
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
