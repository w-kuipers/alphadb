use alphadb::AlphaDB as AlphaDBCore;
use pyo3::prelude::*;

#[pyclass]
struct AlphaDBPythonWrapper(AlphaDBCore);

#[pymethods]
impl AlphaDBPythonWrapper {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|_token| AlphaDBPythonWrapper(AlphaDBCore {..})
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn alphadb_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
