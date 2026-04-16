use actix_web::{
    HttpResponse, Responder, get,
    http::StatusCode,
    post,
    web::{self, Json},
};

use crate::{
    IpAddr, IpStorage, SystemMectrics,
    agent::{get_sys_info, get_sys_metric},
};

#[get("/")]
pub async fn root() -> impl Responder {
    "Hello World"
}

#[get("/api/sysinfo")]
pub async fn sysinfo(query: web::Query<IpAddr>) -> impl Responder {
    let info = get_sys_info(query.0);
    serde_json::to_string(&info.await)
}

#[get("/api/sysmetric")]
pub async fn sysmetric(query: web::Query<IpAddr>) -> impl Responder {
    let metric = get_sys_metric(query.0);
    serde_json::to_string(&metric.await)
}

#[get("/api/getips")]
pub async fn getips(data: web::Data<IpStorage>) -> impl Responder {
    let guard = data.storage.lock().unwrap();
    serde_json::to_string(&*guard).unwrap()
}

#[post("/api/addip")]
pub async fn addip(body: Json<IpAddr>, data: web::Data<IpStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    storage.push(body.0);
    HttpResponse::Ok().status(StatusCode::OK).body("Added Ip")
}

#[get("/api/ping")]
pub async fn ping(body: Json<IpAddr>) -> Json<SystemMectrics> {
    let string = get_sys_metric(body.0);
    Json(string.await)
}
