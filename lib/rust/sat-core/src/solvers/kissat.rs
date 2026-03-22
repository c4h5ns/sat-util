use super::{Solver, SolverOutput};
use crate::{cnf::Cnf, util::ResultExt};
use color_eyre::{
    Result,
    eyre::{Error, OptionExt},
};
use futures::TryFutureExt;
use regex::Regex;
use std::{ops::Deref, process::Stdio, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::Command,
};

pub struct KissatSolverContext {
    process_time_regex: Regex,
    satisfiability_regex: Regex,
    timeout: Duration,
}

impl KissatSolverContext {
    pub fn new(timeout: Duration) -> Result<Self> {
        Ok(Self {
            process_time_regex: Regex::new(r"^c\sprocess-time:\s*.*?(\d*(?:\.\d+)?)\s*seconds$")?,
            satisfiability_regex: Regex::new(r"^s\s+(.*?)\s*$")?,
            timeout,
        })
    }

    pub fn create_solver(self: Arc<Self>, cnf: Arc<Cnf>) -> Result<KissatSolver> {
        Ok(KissatSolver {
            context: self,
            cnf: cnf.clone(),
        })
    }
}

#[derive(Clone)]
pub struct KissatSolver {
    context: Arc<KissatSolverContext>,
    cnf: Arc<Cnf>,
}

impl Solver for KissatSolver {
    async fn solve(self) -> Result<SolverOutput> {
        let mut child = Command::new("./kissat-4.0.1-linux-amd64")
            .arg("--verbose=0")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdin_handle: tokio::task::JoinHandle<Result<_>> = {
            let cnf = self.cnf.clone();

            let stdin = child
                .stdin
                .take()
                .ok_or_eyre("Unable to take stdin handle")?;
            let mut writer = BufWriter::new(stdin);

            tokio::task::spawn(async move {
                writer.write_all(cnf.to_dimacs_string().as_bytes()).await?;
                writer.flush().await?;

                Ok(())
            })
        };

        let stdout_handle: tokio::task::JoinHandle<Result<_>> = {
            let cnf = self.cnf.clone();
            let context = self.context.clone();
            let stdout = child
                .stdout
                .take()
                .ok_or_eyre("Unable to take stdout handle")?;
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            tokio::task::spawn(async move {
                let mut errors = Vec::<Error>::new();
                let mut process_time = None;
                let mut is_satisfiable = None;

                loop {
                    let line = lines
                        .next_line()
                        .await?
                        .ok_or_eyre("Unable to get neccesary information from the output")?;

                    let KissatSolverContext {
                        process_time_regex,
                        satisfiability_regex,
                        ..
                    } = context.deref();

                    if let Some(v) = process_time
                        .is_none()
                        .then_some(())
                        .and_then(|_| process_time_regex.captures(&line))
                        .and_then(|captures| captures.get(1))
                        .and_then(|matched| matched.as_str().parse::<f64>().take_err(&mut errors))
                    {
                        process_time = Some(Duration::from_secs_f64(v));
                    }

                    if let Some(v) = is_satisfiable
                        .is_none()
                        .then_some(())
                        .and_then(|_| satisfiability_regex.captures(&line))
                        .and_then(|captures| captures.get(1))
                        .map(|matched| matched.as_str())
                    {
                        is_satisfiable = Some(v == "SATISFIABLE");
                    }

                    if process_time.is_some() && is_satisfiable.is_some() {
                        break;
                    }
                }

                Ok(SolverOutput {
                    solver: "kissat",
                    cnf,
                    process_time,
                    is_satisfiable,
                    errors,
                })
            })
        };

        let stderr_handle: tokio::task::JoinHandle<Result<_>> = {
            let stderr = child
                .stderr
                .take()
                .ok_or_eyre("Unable to take stderr handle")?;
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            tokio::task::spawn(async move {
                while let Some(line) = lines.next_line().await? {
                    eprintln!("{}", line);
                }

                Ok(())
            })
        };
        let joined_handles = tokio::task::spawn(async {
            stdin_handle.await??;
            stderr_handle.await??;
            stdout_handle.await?
        })
        .err_into::<color_eyre::eyre::Report>();

        let output = tokio::time::timeout(self.context.timeout, joined_handles)
            .err_into::<color_eyre::eyre::Report>()
            .await???;

        Ok(output)
    }
}
