use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub t: num_bigint::BigUint,
    pub n : num_bigint::BigUint,
    pub g : num_bigint::BigUint,
    pub h : num_bigint::BigUint,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Puzzle {
    pub u: num_bigint::BigUint,
    pub v : num_bigint::BigUint,
}
