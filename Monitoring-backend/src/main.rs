use actix_cors::Cors;
use actix_web::{App, HttpServer, Responder, get};
use monitoring_backend_rs::{SystemInfo, SystemMectrics};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8081;

    println!("Starting server on port {port}");

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(root)
            .service(sysinfo)
            .service(sysmetric)
            .service(getips)
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
async fn getips() -> impl Responder {
    "Hello World"
}
