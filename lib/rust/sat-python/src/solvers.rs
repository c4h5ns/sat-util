mod fixstars;
mod kissat;
use pyo3::prelude::*;
use sat_core::solvers::SolverOutput;

#[pyclass(name = "SolverOutput")]
pub struct PySolverOutput {
    inner: SolverOutput,
}

#[pymethods]
impl PySolverOutput {
    pub fn to_csv_record(&self) -> PyResult<Vec<String>> {
        Ok(self.inner.to_csv_record())
    }

    #[getter]
    pub fn get_is_satisfiable(&self) -> PyResult<Option<bool>> {
        Ok(self.inner.is_satisfiable)
    }

    #[getter]
    pub fn get_process_time(&self) -> PyResult<Option<f64>> {
        Ok(self.inner.process_time.map(|v| v.as_secs_f64()))
    }

    #[getter]
    pub fn get_errors(&self) -> PyResult<Vec<String>> {
        Ok(self.inner.errors.iter().map(|e| e.to_string()).collect())
    }
}

#[pymodule(name = "solvers")]
pub mod py_solvers {
    #[pymodule_export]
    use super::{PySolverOutput, fixstars::py_fixstars, kissat::py_kissat};
}
