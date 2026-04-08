use std::{net::UdpSocket, sync::Mutex, time::Duration};

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    post,
    web::{self, Json},
};
use monitoring_backend_rs::{IpAddr, SystemInfo, SystemMectrics};

struct IpStorage {
    storage: Mutex<Vec<IpAddr>>,
    udp_sock: Mutex<UdpSocket>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8081;
    let udp_socket = UdpSocket::bind("127.0.0.1:9090").unwrap();
    udp_socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let storage = web::Data::new(IpStorage {
        storage: Mutex::new(Vec::new()),
        udp_sock: Mutex::new(udp_socket),
    });

    println!("Starting server on port {port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(storage.clone())
            .service(root)
            .service(sysinfo)
            .service(sysmetric)
            .service(getips)
            .service(addip)
            .service(ping)
    })
    .workers(5)
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn root() -> impl Responder {
    "Hello World"
}

#[get("/api/sysinfo")]
async fn sysinfo() -> impl Responder {
    let sysinfo = SystemInfo::new(78232428274, 2622927572859234);
    serde_json::to_string(&sysinfo)
}
#[get("/api/sysmetric")]
async fn sysmetric() -> impl Responder {
    let sysmetric: SystemMectrics = SystemMectrics::new(58232428274, 2622927572859234, 50);
    serde_json::to_string(&sysmetric)
}

#[get("/api/getips")]
async fn getips(data: web::Data<IpStorage>) -> impl Responder {
    let guard = data.storage.lock().unwrap();
    serde_json::to_string(&*guard).unwrap()
}

#[post("/api/addip")]
async fn addip(body: Json<IpAddr>, data: web::Data<IpStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    storage.push(body.0);
    HttpResponse::Ok().status(StatusCode::OK).body("Added Ip")
}

#[get("/api/ping")]
async fn ping(body: Json<IpAddr>, data: web::Data<IpStorage>) -> impl Responder {
    let udp_socket = data.udp_sock.lock().unwrap();
    let ip = body.0;

    udp_socket
        .send_to("type=smetric;".as_bytes(), format!("{}:{}", ip.ip, ip.port))
        .expect("faild to send");

    let mut buf = [0; 200];
    let (amount, _) = udp_socket.recv_from(&mut buf).unwrap();
    let str = String::from_utf8_lossy(&buf[..amount]);
    let res = SystemMectrics::from_agent_response(&str);
    serde_json::to_string(&res)
}
