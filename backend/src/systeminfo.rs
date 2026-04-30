use serde::{Deserialize, Serialize};

/// Used to store the values from an agent response.
/// Most commonly use with the from_agent_response method.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SystemInfo {
    mem_total: u64,
    disk_total: u64,
    distro: String,
    hostname: String,
}

impl SystemInfo {
    /// Used to create a new instance of the SystemInfo struct.
    /// # Common usage
    /// Most commonly used to create a empty struct or for getting som filler data
    pub fn new(mem_total: u64, disk_total: u64, distro: String, hostname: String) -> SystemInfo {
        SystemInfo {
            mem_total,
            disk_total,
            distro,
            hostname,
        }
    }

    /// Returns a SystemInfo struct from the given string.
    /// This will return a corresponding SystemInfo Struct
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pars_systeminfo() {
        let agent_responce = "type=response;mem=434;disk=5000;distro=Arch Linux;hostname=Host";

        let res = SystemInfo::from_agent_response(agent_responce);

        assert_eq!(
            res,
            SystemInfo::new(434, 5000, "Arch Linux".to_owned(), "Host".to_owned())
        )
    }
}
