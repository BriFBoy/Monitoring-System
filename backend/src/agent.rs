use std::time::Duration;
use std::{error::Error, net::UdpSocket};

use actix_web::web::block;

use crate::database::IPaddr;
use crate::systeminfo::SystemInfo;
use crate::systemmetrics::SystemMectrics;

/// Sends a response to a given agent to get the SystemInfo.
/// # Errors
/// - It will want for a maximux of 1 second before givening and error.
/// - It will error if it is unable to create a new udp Socket.
/// - It will error if it is unable to send or resceve from the socket.
/// # Returns
/// It will return a Result with a UTF8 string of what it receved form the socket.
/// If the sender does not return a stirng the function will still try to pars it to UTF8
pub async fn get_sys_info(ip: IPaddr) -> Result<SystemInfo, Box<dyn Error + Send + Sync>> {
    block(move || {
        let udp_socket = UdpSocket::bind("0.0.0.0:0")?;
        udp_socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        udp_socket.send_to("type=info;".as_bytes(), format!("{}:{}", ip.ip, ip.port))?;

        let mut buf = [0; 200];
        let (amount, _) = udp_socket.recv_from(&mut buf)?;
        let str = String::from_utf8_lossy(&buf[..amount]);
        Ok(SystemInfo::from_agent_response(&str))
    })
    .await?
}

/// Sends a response to a given agent to get the SystemMectric.
/// # Errors
/// - It will want for a maximux of 1 second before givening and error.
/// - It will error if it is unable to create a new udp Socket.
/// - It will error if it is unable to send or resceve from the socket.
/// # Returns
/// It will return a Result with a UTF8 string of what it receved form the socket.
/// If the sender does not return a stirng the function will still try to pars it to UTF8
pub async fn get_sys_metric(ip: IPaddr) -> Result<SystemMectrics, Box<dyn Error + Send + Sync>> {
    actix_web::web::block(move || {
        let udp_socket = UdpSocket::bind("0.0.0.0:0")?;
        udp_socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        udp_socket.send_to("type=metric;".as_bytes(), format!("{}:{}", ip.ip, ip.port))?;

        let mut buf = [0; 200];
        let (amount, _) = udp_socket.recv_from(&mut buf)?;
        let str = String::from_utf8_lossy(&buf[..amount]);
        Ok(SystemMectrics::from_agent_response(&str))
    })
    .await?
}
