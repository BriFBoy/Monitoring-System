use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    web::{self, Data},
};
use monitoring_backend_rs::api::{add_ip, get_ips, ping, sys_info, sys_metric};
use monitoring_backend_rs::{IpStorage, api::delete_ip};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8081;
    let storage = Data::new(IpStorage {
        storage: Mutex::new(Vec::new()),
    });
    let sock = format!("0.0.0.0:{port}");

    println!("Starting server on {sock}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new().wrap(cors).app_data(storage.clone()).service(
            web::scope("/api")
                .service(sys_info)
                .service(sys_metric)
                .service(get_ips)
                .service(add_ip)
                .service(ping)
                .service(delete_ip),
        )
    })
    .workers(5)
    .bind(sock)
    .unwrap()
    .run()
    .await
}
