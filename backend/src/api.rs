use actix_web::{
    HttpResponse, Responder, delete, get,
    http::StatusCode,
    post,
    web::{self, Json},
};

use crate::{
    IpAddr, IpStorage, SystemInfo, SystemMectrics,
    agent::{get_sys_info, get_sys_metric},
};

#[get("/sysinfo")]
pub async fn sys_info(query: web::Query<IpAddr>) -> impl Responder {
    let info = get_sys_info(query.0);
    serde_json::to_string(&info.await.unwrap_or(SystemInfo::new(0, 0)))
}

#[get("/sysmetric")]
pub async fn sys_metric(query: web::Query<IpAddr>) -> impl Responder {
    let metric = get_sys_metric(query.0);
    serde_json::to_string(&metric.await.unwrap_or(SystemMectrics::new(0, 0, 0)))
}

#[get("/getips")]
pub async fn get_ips(data: web::Data<IpStorage>) -> impl Responder {
    let guard = data.storage.lock().unwrap();
    serde_json::to_string(&*guard).unwrap()
}

#[post("/addip")]
pub async fn add_ip(body: Json<IpAddr>, data: web::Data<IpStorage>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();
    storage.push(body.0);
    HttpResponse::Ok().status(StatusCode::OK).body("Added Ip")
}

#[get("/ping")]
pub async fn ping(_body: Json<IpAddr>) -> impl Responder {
    "Ping the agent"
}

#[delete("/ping")]
pub async fn delete_ip(_body: Json<IpAddr>) -> impl Responder {
    "todo"
}
