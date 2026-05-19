use serde::{Deserialize, Serialize};

pub mod crypto;
pub mod lockdown;
pub mod ai;
pub mod biometrics;
pub mod database;

#[cfg(test)]
mod crypto_tests;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogicGate {
    pub id: String,
    pub difficulty: u32,
    pub metadata: MetaData,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub r#type: String,
    pub axis: String,
}
