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

#[cfg(all(feature = "mysql", feature = "postgres"))]
compile_error!("Enable only one database engine feature: mysql or postgres");

#[cfg(not(any(feature = "mysql", feature = "postgres")))]
compile_error!("Enable one database engine feature: mysql or postgres");

use alphadb::core::method_types::{Init, Query as AdbQuery};
use alphadb::prelude::*;
#[cfg(all(feature = "mysql", not(feature = "postgres")))]
use mysql::PooledConn;
#[cfg(all(feature = "postgres", not(feature = "mysql")))]
use postgres::Client;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[cfg(any(
    all(feature = "mysql", feature = "postgres"),
    not(any(feature = "mysql", feature = "postgres"))
))]
struct DisabledConnection;

#[cfg(all(feature = "mysql", not(feature = "postgres")))]
type DbConnection = PooledConn;
#[cfg(all(feature = "postgres", not(feature = "mysql")))]
type DbConnection = Client;
#[cfg(any(
    all(feature = "mysql", feature = "postgres"),
    not(any(feature = "mysql", feature = "postgres"))
))]
type DbConnection = DisabledConnection;

#[cfg(all(feature = "mysql", not(feature = "postgres")))]
const DEFAULT_PORT: u16 = 3306;
#[cfg(all(feature = "postgres", not(feature = "mysql")))]
const DEFAULT_PORT: u16 = 5432;
#[cfg(any(
    all(feature = "mysql", feature = "postgres"),
    not(any(feature = "mysql", feature = "postgres"))
))]
const DEFAULT_PORT: u16 = 0;

#[pyclass(unsendable)]
struct AlphaDB {
    inner: alphadb::AlphaDB<DbConnection>,
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
            data: q.data.map(|data| {
                data.into_iter()
                    .map(|value| value.to_string_lossy())
                    .collect()
            }),
            query: q.query,
        }
    }
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
enum PyToleratedVerificationIssueLevel {
    /// Low: Will pass with verification errors below level high.
    Low,
    /// High: Will pass with verification errors below level Critical.
    High,
    /// Critical: Will ignore all errors.
    Critical,
    /// All: Will fail with an error of any level.
    All,
}

#[pymethods]
impl AlphaDB {
    #[new]
    fn __new__() -> Self {
        #[cfg(all(feature = "mysql", not(feature = "postgres")))]
        let config = alphadb::engine::mysql();

        #[cfg(all(feature = "postgres", not(feature = "mysql")))]
        let config = alphadb::engine::postgres();

        #[cfg(any(
            all(feature = "mysql", feature = "postgres"),
            not(any(feature = "mysql", feature = "postgres"))
        ))]
        let config = unreachable!();

        Self {
            inner: alphadb::AlphaDB::new(config),
        }
    }

    #[getter]
    fn is_connected(&self) -> bool {
        self.inner.is_connected
    }

    #[pyo3(signature = (host, user, password, database, port=DEFAULT_PORT))]
    fn connect(
        &mut self,
        host: &str,
        user: &str,
        password: &str,
        database: &str,
        port: u16,
    ) -> PyResult<()> {
        match self.inner.connect(host, user, password, database, port) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }

    fn init(&mut self) -> PyResult<()> {
        match self.inner.init() {
            Ok(i) => match i {
                Init::AlreadyInitialized => Err(PyRuntimeError::new_err(
                    "The database is already initialized",
                )),
                Init::Success => Ok(()),
            },
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }

    fn status(&mut self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| match self.inner.status() {
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

    #[pyo3(signature = (version_source, target_version=None, no_data=false))]
    fn update_queries(
        &mut self,
        version_source: String,
        target_version: Option<&str>,
        no_data: bool,
    ) -> PyResult<Vec<Query>> {
        Python::with_gil(|_py| {
            match self
                .inner
                .update_queries(version_source, target_version, no_data)
            {
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

    #[pyo3(signature = (version_source, target_version=None, no_data=false, verify=true, tolerated_verification_issue_level=PyToleratedVerificationIssueLevel::Low))]
    fn update(
        &mut self,
        version_source: String,
        target_version: Option<String>,
        no_data: Option<bool>,
        verify: Option<bool>,
        tolerated_verification_issue_level: PyToleratedVerificationIssueLevel,
    ) -> PyResult<()> {
        let allowed_error_priority = match tolerated_verification_issue_level {
            PyToleratedVerificationIssueLevel::Low => ToleratedVerificationIssueLevel::Low,
            PyToleratedVerificationIssueLevel::High => ToleratedVerificationIssueLevel::High,
            PyToleratedVerificationIssueLevel::Critical => {
                ToleratedVerificationIssueLevel::Critical
            }
            PyToleratedVerificationIssueLevel::All => ToleratedVerificationIssueLevel::All,
        };

        let no_data = match no_data {
            Some(nd) => nd,
            None => false,
        };

        let verify = match verify {
            Some(v) => v,
            None => false,
        };

        match self.inner.update(
            version_source,
            target_version.as_deref(),
            verify,
            no_data,
            allowed_error_priority,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyRuntimeError::new_err(e.message())),
        }
    }

    fn vacate(&mut self) -> PyResult<()> {
        match self.inner.vacate() {
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
