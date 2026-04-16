use std::{error::Error, net::UdpSocket};

use actix_web::web::block;

use crate::{IpAddr, SystemInfo, SystemMectrics};

pub async fn get_sys_info(ip: IpAddr) -> Result<SystemInfo, Box<dyn Error + Send + Sync>> {
    block(move || {
        let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

        udp_socket.send_to("type=info;".as_bytes(), format!("{}:{}", ip.ip, ip.port))?;

        let mut buf = [0; 200];
        let (amount, _) = udp_socket.recv_from(&mut buf)?;
        let str = String::from_utf8_lossy(&buf[..amount]);
        Ok(SystemInfo::from_agent_response(&str))
    })
    .await?
}

pub async fn get_sys_metric(ip: IpAddr) -> Result<SystemMectrics, Box<dyn Error + Send + Sync>> {
    actix_web::web::block(move || {
        let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

        udp_socket.send_to("type=metric;".as_bytes(), format!("{}:{}", ip.ip, ip.port))?;

        let mut buf = [0; 200];
        let (amount, _) = udp_socket.recv_from(&mut buf)?;
        let str = String::from_utf8_lossy(&buf[..amount]);
        Ok(SystemMectrics::from_agent_response(&str))
    })
    .await?
}
