use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use axum::extract::State;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct WSSystemJson {
    pub used_memory: u64,
    pub used_swap: u64,
    pub cpus_usage: Vec<f32>,
    pub total_swap: u64,
    pub total_memory: u64
    // pub nw_packets_recieved: u64,
    // pub nw_packets_transmitted: u64,
}

//#[derive(Clone, Deserialize, Serialize, Debug)]
//pub struct StaticSystemJson {
//    pub total_swap: u64,
//    pub total_memory: u64,
//}