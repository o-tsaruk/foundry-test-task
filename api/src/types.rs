use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ValuesType {
    Amount,
    Percentage,
}

#[derive(Deserialize)]
pub struct DisperseRequest {
    pub values: Vec<u128>,
    pub total_amount: Option<u128>,
    pub values_type: ValuesType,
}

#[derive(Deserialize)]
pub struct CollectRequest {
    pub values: Vec<u128>,
    pub total_amount: Option<u128>,
    pub values_type: ValuesType,
}
