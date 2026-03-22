use sat_core::solvers::{
    Solver,
    fixstars::{FixstarsSolver, FixstarsSolverContext},
};

use crate::{cnf::PyCnf, solvers::PySolverOutput};
use color_eyre::eyre::OptionExt;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(name = "FixstarsSolverContext")]
struct PyFixstarsSolverContext {
    inner: Arc<FixstarsSolverContext>,
}

#[pymethods]
impl PyFixstarsSolverContext {
    #[new]
    fn __new__(access_token: String) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(FixstarsSolverContext::new(access_token)?),
        })
    }

    pub fn create_solver(slf: Bound<Self>, cnf: Bound<PyCnf>) -> PyResult<PyFixstarsSolver> {
        let inner = slf
            .borrow()
            .inner
            .clone()
            .create_solver(cnf.borrow().inner.clone())?;

        Ok(PyFixstarsSolver { inner: Some(inner) })
    }
}

#[pyclass(name = "FixstarsSolver")]
struct PyFixstarsSolver {
    pub inner: Option<FixstarsSolver>,
}

#[pymethods]
impl PyFixstarsSolver {
    fn solve<'py>(slf: Bound<'py, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
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

#[pymodule(name = "fixstars")]
pub mod py_fixstars {
    #[pymodule_export]
    use super::{PyFixstarsSolver, PyFixstarsSolverContext};
}
