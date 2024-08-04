use alphadb::{AlphaDB as AlphaDBCore};
use pyo3::{prelude::*, types::PyDict, Python};
use pyo3::types::IntoPyDict;

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

    // let key_vals: &[(&str, PyObject)] = [ ("num", 8.to_object()), ("str", "asd".to_object()) ]
    // let dict = key_vals.into_py_dict(py);

    fn check<'a>(&'a mut self, py: &'a Python) -> Bound<PyDict> {
        let check = self.alphadb_instance.check();
        let key_val: Vec<(&str, PyObject)> = vec![("check", check.check.to_object(py)), ("version", check.version.to_object(py))];

        return key_val.into_py_dict_bound(*py);
    }
}

#[pymodule(name = "alphadb")]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlphaDB>()?;
    Ok(())
}
