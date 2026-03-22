mod cnf;
mod solvers;

use pyo3::prelude::*;

#[pymodule(name = "sat_python")]
pub mod py_sat_python {
    #[pymodule_export]
    use crate::{cnf::py_cnf, solvers::py_solvers};
}
