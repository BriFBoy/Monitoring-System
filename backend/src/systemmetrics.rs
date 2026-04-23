use serde::{Deserialize, Serialize};

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
    pub fn from_agent_response(raw: &str) -> Self {
        let mut mem_used = 0u64;
        let mut disk_used = 0u64;
        let mut cpu_usage = 0u8;
        raw.split(";").for_each(|pair| {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                return;
            }
            let (key, value) = (parts[0].trim(), parts[1].trim());
            match key {
                "mem" => mem_used = value.parse().unwrap_or(0),
                "disk" => disk_used = value.parse().unwrap_or(0),
                "cpu" => cpu_usage = value.parse().unwrap_or(0),
                _ => {}
            }
        });
        SystemMectrics::new(mem_used, disk_used, cpu_usage)
    }
}
