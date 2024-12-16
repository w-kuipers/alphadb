// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty ofprintln
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use alphadb::methods::connect::connect;
use alphadb::methods::init::init;
use alphadb::methods::status::status;
use alphadb::methods::update_queries::update_queries;
use alphadb::methods::update_queries::Query as AdbQuery;
use alphadb::methods::vacate::vacate;
use alphadb::prelude::*;
use mysql::PooledConn;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass]
struct AlphaDB {
    connection: Option<PooledConn>,
    db_name: Option<String>,
}

#[derive(Debug, IntoPyObject, IntoPyObjectRef)]
pub struct Status {
    pub init: bool,
    pub version: Option<String>,
    pub name: String,
    pub template: Option<String>,
}

#[derive(Debug, IntoPyObject, IntoPyObjectRef)]
pub struct Query {
    pub query: String,
    pub data: Option<Vec<String>>,
}

impl From<AdbQuery> for Query {
    fn from(q: AdbQuery) -> Self {
        Query {
            data: q.data,
            query: q.query,
        }
    }
}

#[pymethods]
impl AlphaDB {
    #[new]
    fn __new__() -> Self {
        Self {
            connection: None,
            db_name: None,
        }
    }

    #[pyo3(signature = (host, user, password, database, port=3306))]
    fn connect(
        &mut self,
        host: String,
        user: String,
        password: String,
        database: String,
        port: u16,
    ) -> PyResult<()> {
        match connect(&host, &user, &password, &database, &port) {
            Ok(c) => {
                self.connection = Some(c);
                self.db_name = Some(database);
                Ok(())
            }
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }

    fn init(&mut self) -> PyResult<()> {
        match init(&self.db_name, &mut self.connection) {
            Ok(i) => match i {
                alphadb::Init::AlreadyInitialized => Err(PyRuntimeError::new_err(
                    "The database is already initialized",
                )),
                alphadb::Init::Success => Ok(()),
            },
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }

    fn status(&mut self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| match status(&self.db_name, &mut self.connection) {
            Ok(s) => {
                let status = Status {
                    init: s.init,
                    version: s.version,
                    name: s.name,
                    template: s.template,
                }
                .into_pyobject(py);

                match status {
                    Ok(status) => Ok(status.into()),
                    Err(_) => Err(PyRuntimeError::new_err("Unable to parse return value")),
                }
            }
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        })
    }

    #[pyo3(signature = (version_source, update_to_version=None))]
    fn update_queries(
        &mut self,
        version_source: String,
        update_to_version: Option<&str>,
    ) -> PyResult<Vec<Query>> {
        Python::with_gil(|_py| {
            match update_queries(
                &self.db_name,
                &mut self.connection,
                version_source,
                update_to_version,
            ) {
                Ok(queries) => {
                    let mut queries_converted: Vec<Query> = Vec::new();

                    for query in queries {
                        queries_converted.push(query.into());
                    }

                    Ok(queries_converted)
                }
                Err(e) => Err(PyRuntimeError::new_err(e.message())),
            }
        })
    }

    // #[pyo3(signature = (version_source, update_to_version=None, no_data=false, verify=true, allowed_error_priority=PyVerificationIssueLevel::Low))]
    // fn update(
    //     &mut self,
    //     version_source: String,
    //     update_to_version: Option<String>,
    //     no_data: Option<bool>,
    //     verify: Option<bool>,
    //     allowed_error_priority: PyVerificationIssueLevel,
    // ) {
    //     let mut no_data_wrapper = false;
    //     let mut verify_wrapper = true;
    //     let allowed_error_priority_wrapper: VerificationIssueLevel;
    //
    //     if no_data.is_some() {
    //         no_data_wrapper = no_data.unwrap();
    //     }
    //
    //     if verify.is_some() {
    //         verify_wrapper = verify.unwrap();
    //     }
    //
    //     if let PyVerificationIssueLevel::Low { .. } = allowed_error_priority {
    //         allowed_error_priority_wrapper = VerificationIssueLevel::Low;
    //     } else if let PyVerificationIssueLevel::High { .. } = allowed_error_priority {
    //         allowed_error_priority_wrapper = VerificationIssueLevel::High;
    //     } else {
    //         allowed_error_priority_wrapper = VerificationIssueLevel::Critical;
    //     }
    //
    //     self.alphadb_instance.update(
    //         version_source,
    //         update_to_version,
    //         verify_wrapper,
    //         no_data_wrapper,
    //         allowed_error_priority_wrapper,
    //     );
    //
    // }

    fn vacate(&mut self) -> PyResult<()> {
        match vacate(&mut self.connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }
}

#[pymodule(name = "alphadb")]
fn alphadb_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AlphaDB>()?;
    Ok(())
}
