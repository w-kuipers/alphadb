use alphadb::AlphaDB;
use pyo3::prelude::*;

// #[pyclass]
// struct AlphaDB {
//     connection: Option<Pooled>,
// }

/// A Python module implemented in Rust.
#[pymodule]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
