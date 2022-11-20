use processing_chain::run_process_json;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn run_process(process_name: str, json_file_path: str, lambda: PyObject) -> PyResult<()> {
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_processing_chain(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_process, m)?)?;
    Ok(())
}