use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub energies: Vec<f32>,
    pub execution_parameters: ExecutionParameters,
    pub execution_time: ExecutionTime,
    pub feasibilities: Vec<bool>,
    pub message: String,
    pub spins: Vec<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionParameters {
    pub timeout: f32,
    pub num_gpus: i32,
    pub num_iterations: i32,
    pub penalty_calibration: bool,
    pub penalty_multipliers: Vec<f32>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionTime {
    pub annealing_time: f32,
    pub queue_time: f32,
    pub cpu_time: f32,
    pub time_stamps: Vec<f32>,
}
