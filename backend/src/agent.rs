use std::net::UdpSocket;

use crate::{IpAddr, SystemMectrics};

pub fn get_agent_metrics(ipaddr: IpAddr) -> Result<SystemMectrics, std::io::Error> {
    let udp_sock = UdpSocket::bind("0.0.0.0:8081")?;
    udp_sock.send_to(
        "type=smetric;".as_bytes(),
        format!("{}:{}", ipaddr.ip, ipaddr.port),
    )?;
    let mut buf = [0; 20];
    let (amount, _) = udp_sock.recv_from(&mut buf)?;
    let response = String::from_utf8_lossy(&buf[..amount]);
    Ok(SystemMectrics::from_agent_response(&response))
}
