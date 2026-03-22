pub mod models;

use super::{Solver, SolverOutput};
use crate::{cnf::Cnf, util::ResultExt};
use color_eyre::{Result, eyre::eyre};
use reqwest::RequestBuilder;
use std::{sync::Arc, time::Duration};

pub struct FixstarsSolverContext {
    #[allow(dead_code)]
    access_token: String,
    request_builder: RequestBuilder,
}

impl FixstarsSolverContext {
    pub fn new(access_token: String) -> Result<Self> {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str("*/*")?,
        );
        default_headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))?,
        );
        default_headers.insert(
            reqwest::header::CONNECTION,
            reqwest::header::HeaderValue::from_str("close")?,
        );
        default_headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_str("optigan.fixstars.com")?,
        );
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str("FixstarsAmplify/1.3.1")?,
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .gzip(true)
            .build()?;
        let request_builder = client.post("https://optigan.fixstars.com/solve");

        Ok(Self {
            access_token,
            request_builder,
        })
    }

    pub fn create_solver(self: Arc<Self>, cnf: Arc<Cnf>) -> Result<FixstarsSolver> {
        let request_body = models::request::RequestBody {
            outputs: models::request::Outputs {},
            timeout: 10000,
            constraints: generate_constraints(&cnf),
            polynomial: vec![],
        };

        let request_builder = self
            .request_builder
            .try_clone()
            .ok_or(eyre!("Unable to clone request builder"))?
            .json(&request_body);

        Ok(FixstarsSolver {
            context: self,
            cnf,
            request_builder,
        })
    }
}

pub struct FixstarsSolver {
    #[allow(dead_code)]
    context: Arc<FixstarsSolverContext>,
    cnf: Arc<Cnf>,
    request_builder: RequestBuilder,
}

impl Solver for FixstarsSolver {
    async fn solve(self) -> Result<SolverOutput> {
        let mut errors = Vec::new();
        let response = self.request_builder.send().await?;

        let response_body = response.json::<models::response::ResponseBody>().await?;
        let process_time = response_body
            .execution_time
            .time_stamps
            .get(0)
            .ok_or_else(|| eyre!("Unable to read process time in output"))
            .take_err(&mut errors)
            .map(|process_time| Duration::from_secs_f32(process_time / 1e3));
        let is_satisfiable = Some(response_body.feasibilities.iter().any(|x| *x));

        Ok(SolverOutput {
            solver: "fixstars",
            cnf: self.cnf,
            process_time,
            is_satisfiable,
            errors,
        })
    }
}

fn generate_constraints(cnf: &Cnf) -> Vec<models::request::Constraint> {
    let constraints_clause = (0..cnf.num_clauses).map(|i| {
        let start = i * cnf.num_literals;
        let end = start + cnf.num_literals;

        let condition = models::request::Condition {
            left: (start..end)
                .map(|q| models::request::Term::XK((q, 1.0)))
                .collect(),
            op: "EQ".to_owned(),
            right: 1.0,
        };

        let penalty = (start..end)
            .flat_map(|p| {
                (p..end).map(move |q| {
                    if p == q {
                        models::request::Term::XK((p, -1.0))
                    } else {
                        models::request::Term::XYK((p, q, 2.0))
                    }
                })
            })
            .chain([models::request::Term::K((1.0,))])
            .collect();

        models::request::Constraint {
            condition,
            penalty,
            multiplier: 1.0,
        }
    });

    let constraint_contradictive = itertools::iproduct!(
        0..cnf.num_clauses,
        0..cnf.num_literals,
        0..cnf.num_clauses,
        0..cnf.num_literals,
    )
    .filter(|&(i1, j1, i2, j2)| cnf.formula[[i1, j1]] + cnf.formula[[i2, j2]] == 0)
    .map(|(i1, j1, i2, j2)| {
        let p = i1 * cnf.num_literals + j1;
        let q = i2 * cnf.num_literals + j2;

        let left = vec![models::request::Term::XYK((p, q, 1.0))];

        let condition = models::request::Condition {
            left: left.clone(),
            right: 0.0,
            op: "EQ".to_owned(),
        };

        let penalty = left;

        models::request::Constraint {
            condition,
            penalty,
            multiplier: 1.0,
        }
    });

    constraints_clause.chain(constraint_contradictive).collect()
}
