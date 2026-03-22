use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub constraints: Vec<Constraint>,
    pub outputs: Outputs,
    pub timeout: u32,
    pub polynomial: Vec<Polynomial>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constraint {
    pub condition: Condition,
    pub multiplier: f32,
    pub penalty: Vec<Term>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Outputs {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Polynomial {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub left: Vec<Term>,
    pub op: String,
    pub right: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Term {
    XYK((usize, usize, f32)),
    XK((usize, f32)),
    K((f32,)),
}
