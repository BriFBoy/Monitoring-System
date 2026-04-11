use std::net::UdpSocket;

use crate::{IpAddr, SystemInfo, SystemMectrics};

pub fn get_sys_info(ip: IpAddr) -> SystemInfo {
    let udp_socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    udp_socket
        .send_to("type=info;".as_bytes(), format!("{}:{}", ip.ip, ip.port))
        .expect("faild to send");

    let mut buf = [0; 200];
    let (amount, _) = udp_socket.recv_from(&mut buf).unwrap();
    let str = String::from_utf8_lossy(&buf[..amount]);
    SystemInfo::from_agent_response(&str)
}

pub fn get_sys_metric(ip: IpAddr) -> SystemMectrics {
    let udp_socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    udp_socket
        .send_to("type=metric;".as_bytes(), format!("{}:{}", ip.ip, ip.port))
        .expect("faild to send");

    let mut buf = [0; 200];
    let (amount, _) = udp_socket.recv_from(&mut buf).unwrap();
    let str = String::from_utf8_lossy(&buf[..amount]);
    SystemMectrics::from_agent_response(&str)
}
