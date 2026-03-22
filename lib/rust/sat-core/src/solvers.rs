use crate::cnf::Cnf;
use color_eyre::Result;
use fixstars::FixstarsSolver;
use kissat::KissatSolver;
use std::{sync::Arc, time::Duration};

pub mod fixstars;
pub mod kissat;

pub trait Solver {
    fn solve(self) -> impl std::future::Future<Output = Result<SolverOutput>> + Send;
}

#[derive(Debug)]
pub struct SolverOutput {
    pub solver: &'static str,
    pub cnf: Arc<Cnf>,
    pub is_satisfiable: Option<bool>,
    pub process_time: Option<Duration>,
    pub errors: Vec<color_eyre::eyre::Error>,
}

impl SolverOutput {
    pub const HEADER: [&str; 7] = [
        "solver",
        "num_variables",
        "num_clauses",
        "num_literals",
        "seed",
        "is_satisfiable",
        "process_time",
    ];

    pub fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.solver.to_string(),
            self.cnf.num_variables.to_string(),
            self.cnf.num_clauses.to_string(),
            self.cnf.num_literals.to_string(),
            self.cnf.seed.to_string(),
            self.is_satisfiable
                .map(|v| v.to_string())
                .unwrap_or("".to_string()),
            self.process_time
                .map(|v| v.as_secs_f64().to_string())
                .unwrap_or("".to_string()),
        ]
    }
}

pub enum SolverKind {
    Fixstars(FixstarsSolver),
    Kissat(KissatSolver),
}
