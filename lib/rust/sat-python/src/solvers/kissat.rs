use crate::{cnf::PyCnf, solvers::PySolverOutput};
use color_eyre::eyre::{self, OptionExt, eyre};
use pyo3::prelude::*;
use sat_core::solvers::{
    Solver,
    kissat::{KissatSolver, KissatSolverContext},
};
use std::{sync::Arc, time::Duration};

pub struct DeserializedDuration(Duration);

impl From<Duration> for DeserializedDuration {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}

impl Into<Duration> for DeserializedDuration {
    fn into(self) -> Duration {
        self.0
    }
}

impl<'src, 'py> FromPyObject<'src, 'py> for DeserializedDuration {
    type Error = eyre::ErrReport;

    fn extract(obj: Borrowed<'src, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<String>() {
            Ok(iso8601_duration::Duration::parse(s.as_str())
                .map_err(|_| eyre!("Invalid duration format"))?
                .to_std()
                .ok_or_eyre("Invalid duration format")?
                .into())
        } else if let Ok(d) = obj.extract::<f32>() {
            Ok(Duration::from_secs_f32(d).into())
        } else {
            Err(eyre!("Invalid duration type"))
        }
    }
}

#[pyclass(name = "KissatSolverContext")]
pub struct PyKissatSolverContext {
    pub inner: Arc<KissatSolverContext>,
}

#[pymethods]
impl PyKissatSolverContext {
    #[new]
    pub fn __new__(timeout: DeserializedDuration) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(KissatSolverContext::new(timeout.into())?),
        })
    }

    pub fn create_solver(slf: Bound<Self>, cnf: Bound<PyCnf>) -> PyResult<PyKissatSolver> {
        let inner = slf
            .borrow()
            .inner
            .clone()
            .create_solver(cnf.borrow().inner.clone())?;
        Ok(PyKissatSolver { inner: Some(inner) })
    }
}

#[pyclass(name = "KissatSolver")]
pub struct PyKissatSolver {
    pub inner: Option<KissatSolver>,
}

#[pymethods]
impl PyKissatSolver {
    pub fn solve<'py>(slf: Bound<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = slf
            .borrow_mut()
            .inner
            .take()
            .ok_or_eyre("Solver has already been used or is uninitialized.")?;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let output = inner.solve().await?;
            Ok(PySolverOutput { inner: output })
        })
    }
}

#[pymodule(name = "kissat")]
pub mod py_kissat {
    #[pymodule_export]
    use super::{PyKissatSolver, PyKissatSolverContext};
}
