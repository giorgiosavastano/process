use pyo3::prelude::*;
// use processing_chain::items::Item;

extern crate processing_chain;

/// Run process in parallel.
#[pyfunction]
fn run_process(process_name: &str, json_file_path: &str, lambda: PyObject) -> PyResult<()> 
{
    // let _pr = processing_chain::run_process_json(process_name.to_string(), json_file_path.to_string(), lambda);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_processing_chain(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_process, m)?)?;
    Ok(())
}