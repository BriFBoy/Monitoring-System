use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    http::StatusCode,
    post,
    web::{self, Json},
};
use monitoring_backend_rs::{
    IpAddr, IpStorage, SystemMectrics,
    agent::{get_sys_info, get_sys_metric},
};

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
async fn sysinfo(query: web::Query<IpAddr>) -> impl Responder {
    let info = get_sys_info(query.0);
    serde_json::to_string(&info)
}

#[get("/api/sysmetric")]
async fn sysmetric(query: web::Query<IpAddr>) -> impl Responder {
    let metric = get_sys_metric(query.0);
    serde_json::to_string(&metric)
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
async fn ping(body: Json<IpAddr>) -> Json<SystemMectrics> {
    let string = get_sys_metric(body.0);
    Json(string)
}
