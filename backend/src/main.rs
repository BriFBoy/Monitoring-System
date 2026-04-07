use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    post,
    web::{self, Json},
};
use monitoring_backend_rs::{IpAddr, SystemInfo, SystemMectrics};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IpStorage {
    storage: Mutex<Vec<IpAddr>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8081;
    let storage = web::Data::new(IpStorage {
        storage: Mutex::new(Vec::new()),
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
