use alphadb::AlphaDB as AlphaDBCore;
use pyo3::{prelude::*, Python};
use std::collections::HashMap;

#[pyclass]
struct AlphaDB {
    alphadb_instance: AlphaDBCore,
}

#[pyclass]
pub struct Check {
    pub check: bool,
    pub version: Option<String>,
}

#[pymethods]
impl AlphaDB {
    #[new]
    fn __new__() -> Self {
        Self {
            alphadb_instance: AlphaDBCore::new(),
        }
    }

    #[pyo3(signature = (host, user, password, database, port=3306))]
    fn connect(
        &mut self,
        host: String,
        user: String,
        password: String,
        database: String,
        port: i32,
    ) {
        self.alphadb_instance
            .connect(host, user, password, database, port)
    }

    fn check<'a>(&mut self) -> Py<PyAny> {
        return Python::with_gil(|py: Python| {
            let check = self.alphadb_instance.check();
            let check_value = HashMap::from([
                ("check", check.check.to_object(py)),
                ("version", check.version.to_object(py)),
            ]);

            check_value.to_object(py)
        });
    }
}

#[pymodule(name = "alphadb")]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlphaDB>()?;
    Ok(())
}
