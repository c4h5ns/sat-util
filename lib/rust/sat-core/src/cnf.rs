use itertools::Itertools;
use ndarray;
use rand::{RngExt, SeedableRng, seq::index::sample};

pub enum RngAlgorithm {
    ChaCha8,
    Pcg64,
}

impl RngAlgorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "chacha8" => Some(Self::ChaCha8),
            "pcg64" => Some(Self::Pcg64),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Cnf {
    pub num_clauses: usize,
    pub num_literals: usize,
    pub num_variables: usize,
    pub formula: ndarray::Array2<i32>,
    pub seed: u64,
}

impl Cnf {
    fn create_rng(algorithm: RngAlgorithm, seed: u64) -> Box<dyn rand::Rng> {
        match algorithm {
            RngAlgorithm::ChaCha8 => Box::new(rand_chacha::ChaCha8Rng::seed_from_u64(seed)),
            RngAlgorithm::Pcg64 => Box::new(rand_pcg::Pcg64::seed_from_u64(seed)),
        }
    }

    pub fn new(
        num_variables: usize,
        num_literals: usize,
        num_clauses: usize,
        seed: u64,
        algorithm: RngAlgorithm,
        are_distinct: bool,
    ) -> Cnf {
        let mut rng = Self::create_rng(algorithm, seed);
        let mut formula = ndarray::Array2::<i32>::zeros((num_clauses, num_literals));

        for i in 0..num_clauses {
            if are_distinct {
                let variables = sample(&mut rng, num_variables, num_literals)
                    .into_iter()
                    .map(|v| (v as i32) + 1)
                    .collect_vec();

                for j in 0..num_literals {
                    let variable = variables[j];
                    let sign = 1 - 2 * rng.random_range(0..=1);
                    let literal = sign * variable;
                    formula[[i, j]] = literal;
                }
            } else {
                for j in 0..num_literals {
                    let value = rng.random_range(1..=(num_literals as i32 * 2));
                    let sign = 1 - 2 * (value % 2);
                    let variable = (value + 1) / 2;
                    let literal = sign * variable;
                    formula[[i, j]] = literal;
                }
            }
        }

        Self {
            num_clauses,
            num_literals,
            num_variables,
            formula,
            seed,
        }
    }

    pub fn to_dimacs_string(&self) -> String {
        let mut str = String::new();

        str.push_str("p cnf ");
        str.push_str(self.num_variables.to_string().as_str());
        str.push(' ');
        str.push_str(self.num_clauses.to_string().as_str());
        str.push('\n');

        for i in 0..self.num_clauses {
            for j in 0..self.num_literals {
                str.push_str(self.formula[[i, j]].to_string().as_str());
                str.push(' ');
            }
            str.push_str("0\n");
        }

        str
    }
}
