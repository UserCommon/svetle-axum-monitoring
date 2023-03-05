use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use axum::extract::State;

#[derive(Clone, Deserialize, Serialize)]
pub struct SystemJson {
    pub total_memory: String,
    pub used_memory: String,
    pub total_swap: String,
    pub used_swap: String,
}

impl SystemJson {
    pub fn new() -> Self {
        todo!();
        // Self {
        // total_memory:
        // }
    }
}
