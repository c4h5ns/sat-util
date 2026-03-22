use std::sync::Arc;

use pyo3::{exceptions::PyValueError, prelude::*};
use numpy::{PyArray2, ToPyArray};
use sat_core::cnf::{Cnf, RngAlgorithm};

#[pyclass(name = "Cnf")]
pub struct PyCnf {
    pub inner: Arc<Cnf>,
}

#[pymethods]
impl PyCnf {
    #[new]
    #[pyo3(signature = (num_variables, num_literals, num_clauses, seed, algorithm = "chacha8", are_distinct = true))]
    pub fn __new__(
        num_variables: usize,
        num_literals: usize,
        num_clauses: usize,
        seed: u64,
        algorithm: &str,
        are_distinct: bool,
    ) -> PyResult<Self> {
        let algorithm = RngAlgorithm::from_str(algorithm)
            .ok_or_else(|| PyValueError::new_err("Invalid algorithm"))?;

        Ok(Self {
            inner: Arc::new(Cnf::new(
                num_variables,
                num_literals,
                num_clauses,
                seed,
                algorithm,
                are_distinct,
            )),
        })
    }

    pub fn to_dimacs_string(&self) -> PyResult<String> {
        Ok(self.inner.to_dimacs_string())
    }

    #[getter]
    pub fn get_num_clauses(&self) -> PyResult<usize> {
        Ok(self.inner.num_clauses)
    }

    #[getter]
    pub fn get_num_literals(&self) -> PyResult<usize> {
        Ok(self.inner.num_literals)
    }

    #[getter]
    pub fn get_num_variables(&self) -> PyResult<usize> {
        Ok(self.inner.num_variables)
    }

    #[getter]
    pub fn get_seed(&self) -> PyResult<u64> {
        Ok(self.inner.seed)
    }

    #[getter]
    pub fn get_algorithm(&self) -> PyResult<String> {
        Ok(self.inner.seed.to_string())
    }

    pub fn to_formula<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<i32>>> {
        Ok(self.inner.formula.to_pyarray(py))
    }
}

#[pymodule(name = "cnf")]
pub mod py_cnf {
    #[pymodule_export]
    use super::PyCnf;
}
