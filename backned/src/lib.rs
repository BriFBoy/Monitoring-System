use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SystemInfo {
    mem_total: u64,
    disk_total: u64,
}

impl SystemInfo {
    pub fn new(mem_total: u64, disk_total: u64) -> SystemInfo {
        SystemInfo {
            mem_total,
            disk_total,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SystemMectrics {
    mem_used: u64,
    disk_used: u64,
    cpu_usage: u8,
}

impl SystemMectrics {
    pub fn new(mem_used: u64, disk_used: u64, cpu_usage: u8) -> SystemMectrics {
        SystemMectrics {
            mem_used,
            disk_used,
            cpu_usage,
        }
    }
}
