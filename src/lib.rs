use numpy::PyArrayDyn;
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(format!("Summan är: {}", a + b))
}

/// Maths!
/// This mutates the incoming value. Very unsafe if things change
/// on the Python side.
#[pyfunction]
fn multi_playah(a: f64, x: &PyArrayDyn<f64>) {
    let mut x = unsafe { x.as_array_mut() };
    x *= a;
}
/// A Python module implemented in Rust.
#[pymodule]
fn pappyrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multi_playah, m)?)?;
    Ok(())
}
