use alphadb::utils::types::VerificationIssueLevel;
use alphadb::AlphaDB as AlphaDBCore;
use pyo3::types::PyList;
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

#[pyclass]
#[derive(Clone)]
enum PyVerificationIssueLevel {
    /// LOW: Will work, but will not have any effect on the database
    Low,
    /// HIGH: Will still work, but might produce a different result than desired.
    High,
    /// CRITICAL: Will not execute.
    Critical,
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

    fn status(&mut self) -> Py<PyAny> {
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
    fn update_queries(
        &mut self,
        version_source: String,
        update_to_version: Option<&str>,
    ) -> PyResult<Py<PyAny>> {
        let queries = self
            .alphadb_instance
            .update_queries(version_source, update_to_version);

        Python::with_gil(|py| {
            let queries_py_list: Vec<_> = queries
                .into_iter()
                .map(|query| {
                    let data_py_list = PyList::new_bound(py, query.data);
                    vec![query.query.into_py(py), data_py_list.into()]
                })
                .collect();

            let py_list = PyList::new_bound(py, queries_py_list);

            Ok(py_list.into())
        })
    }

    #[pyo3(signature = (version_source, update_to_version=None, no_data=false, verify=true, allowed_error_priority=PyVerificationIssueLevel::Low))]
    fn update(
        &mut self,
        version_source: String,
        update_to_version: Option<String>,
        no_data: Option<bool>,
        verify: Option<bool>,
        allowed_error_priority: PyVerificationIssueLevel,
    ) {
        let mut no_data_wrapper = false;
        let mut verify_wrapper = true;
        let mut allowed_error_priority_wrapper: VerificationIssueLevel =
            VerificationIssueLevel::Low;

        if no_data.is_some() {
            no_data_wrapper = no_data.unwrap();
        }

        if verify.is_some() {
            verify_wrapper = verify.unwrap();
        }

        if let PyVerificationIssueLevel::Low { .. } = allowed_error_priority {
            allowed_error_priority_wrapper = VerificationIssueLevel::Low;
        } else if let PyVerificationIssueLevel::High { .. } = allowed_error_priority {
            allowed_error_priority_wrapper = VerificationIssueLevel::High;
        } else {
            allowed_error_priority_wrapper = VerificationIssueLevel::Critical;
        }

        self.alphadb_instance.update(
            version_source,
            update_to_version,
            verify_wrapper,
            no_data_wrapper,
            allowed_error_priority_wrapper,
        );
    }
}

#[pymodule(name = "alphadb")]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlphaDB>()?;
    Ok(())
}
