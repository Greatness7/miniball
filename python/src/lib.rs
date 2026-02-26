use pyo3::prelude::*;

use miniball_rs::BoundingSphere;
use numpy::PyReadonlyArray2;

#[pyfunction]
fn compute(points: PyReadonlyArray2<'_, f64>) -> PyResult<([f64; 3], f64)> {
    let points = points.as_array();
    let slice = points.as_slice().unwrap_or_default();
    let points = bytemuck::cast_slice(slice);
    let sphere = BoundingSphere::from_points(&points);
    Ok((sphere.center, sphere.radius))
}

#[pymodule]
fn miniball(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}
