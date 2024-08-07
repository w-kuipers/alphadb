use alphadb::AlphaDB as AlphaDBCore;
use pyo3::types::PyTuple;
use pyo3::{prelude::*, Python};
use std::collections::HashMap;

#[pyclass]
struct AlphaDB {
    pub alphadb_instance: AlphaDBCore,
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

    fn init(&mut self) {
        self.alphadb_instance.init();
    }

    fn status<'a>(&mut self) -> Py<PyAny> {
        return Python::with_gil(|py: Python| {
            let status = self.alphadb_instance.status();

            let status_value = HashMap::from([
                ("init", status.init.to_object(py)),
                ("version", status.version.to_object(py)),
                ("name", self.alphadb_instance.db_name.to_object(py)),
                ("template", status.template.to_object(py)),
            ]);

            status_value.to_object(py)
        });
    }

    #[pyo3(signature = (version_source, update_to_version=None))]
    fn update_queries(&mut self, version_source: String, update_to_version: Option<&str>) {

        #[derive(Clone, Debug)]
        enum QueryValue {
            Query(String),
            Data(Option<Vec<String>>),
        }

        let queries = self
            .alphadb_instance
            .update_queries(version_source, update_to_version);

        let mut queries_converted = Vec::from(Vec::from([queries[0].query]));

        println!("{:?}", queries[0]);

        // for query in queries {
        //     queries_converted.push(Vec::from([
        //         QueryValue::Query(query.query),
        //         QueryValue::Data(query.data),
        //     ]));
        // }

        // Python::with_gil(|py: Python| {
        //     PyTuple::new_bound(py, queries_converted);
        // });
    }
}

#[pymodule(name = "alphadb")]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlphaDB>()?;
    Ok(())
}
