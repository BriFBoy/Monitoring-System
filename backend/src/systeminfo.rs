use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SystemInfo {
    mem_total: u64,
    disk_total: u64,
    distro: String,
    hostname: String,
}

impl SystemInfo {
    pub fn new(mem_total: u64, disk_total: u64, distro: String, hostname: String) -> SystemInfo {
        SystemInfo {
            mem_total,
            disk_total,
            distro,
            hostname,
        }
    }
    pub fn from_agent_response(raw: &str) -> Self {
        let mut mem_total = 0u64;
        let mut disk_total = 0u64;
        let mut distro = String::new();
        let mut hostname = String::new();
        raw.split(";").for_each(|pair| {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                return;
            }
            let (key, value) = (parts[0].trim(), parts[1].trim());
            match key {
                "mem" => mem_total = value.parse().unwrap_or(0),
                "disk" => disk_total = value.parse().unwrap_or(0),
                "distro" => distro = value.to_owned(),
                "hostname" => hostname = value.to_owned(),
                _ => {}
            }
        });
        SystemInfo::new(mem_total, disk_total, distro, hostname)
    }
}
