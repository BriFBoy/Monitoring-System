use serde::{Deserialize, Serialize};

/// Used to store the values from an agent response.
/// Most commonly use with the from_agent_response method.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SystemMectrics {
    mem_used: u64,
    disk_used: u64,
    cpu_usage: u8,
    uptime: u64,
}
impl SystemMectrics {
    /// Used to create a new instance of the SystemInfo struct.
    /// # Common usage
    /// Most commonly used to create a empty struct or for getting som filler data
    pub fn new(mem_used: u64, disk_used: u64, cpu_usage: u8, uptime: u64) -> SystemMectrics {
        SystemMectrics {
            mem_used,
            disk_used,
            cpu_usage,
            uptime,
        }
    }
    /// Returns a SystemMectrics struct from the given string.
    /// This will return a corresponding SystemMectrics struct
    pub fn from_agent_response(raw: &str) -> Self {
        let mut mem_used = 0u64;
        let mut disk_used = 0u64;
        let mut cpu_usage = 0u8;
        let mut uptime = 0u64;
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
                "uptime" => uptime = value.parse().unwrap_or(0),
                _ => {}
            }
        });
        SystemMectrics::new(mem_used, disk_used, cpu_usage, uptime)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pars_systeminfo() {
        let agent_responce = "type=response;mem=434;disk=5000;cpu=68;uptime=3600";

        let res = SystemMectrics::from_agent_response(agent_responce);

        assert_eq!(res, SystemMectrics::new(434, 5000, 68, 3600))
    }
}
