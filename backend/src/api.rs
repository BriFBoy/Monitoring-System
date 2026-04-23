use actix_web::{
    HttpResponse, Responder, delete, get,
    http::StatusCode,
    post,
    web::{self, Json},
};

use crate::systeminfo::SystemInfo;
use crate::systemmetrics::SystemMectrics;
use crate::{
    AppData,
    agent::{get_sys_info, get_sys_metric},
    database::IPaddr,
};

#[get("/sysinfo")]
pub async fn sys_info(query: web::Query<IPaddr>) -> impl Responder {
    let info = get_sys_info(query.0);
    serde_json::to_string(&info.await.unwrap_or(SystemInfo::new(
        0,
        0,
        String::new(),
        String::new(),
    )))
}

#[get("/sysmetric")]
pub async fn sys_metric(query: web::Query<IPaddr>) -> impl Responder {
    let metric = get_sys_metric(query.0);
    serde_json::to_string(&metric.await.unwrap_or(SystemMectrics::new(0, 0, 0, 0)))
}

#[get("/getips")]
pub async fn get_ips(data: web::Data<AppData>) -> impl Responder {
    let ips = IPaddr::get_all_ips(&data.db).await;
    serde_json::to_string(&ips)
}

#[post("/addip")]
pub async fn add_ip(body: Json<IPaddr>, data: web::Data<AppData>) -> impl Responder {
    match body.0.insert_ipaddr(&data.db).await {
        Ok(_) => HttpResponse::Ok().status(StatusCode::OK).body("Added Ip"),
        Err(_) => HttpResponse::InternalServerError().body("Error Adding IP"),
    }
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    "Ping the agent"
}

#[delete("/deleteip")]
pub async fn delete_ip(body: Json<IPaddr>, data: web::Data<AppData>) -> impl Responder {
    match body.0.delete_ip(&data.db).await {
        Ok(_) => HttpResponse::Ok().status(StatusCode::OK).body("Added Ip"),
        Err(_) => HttpResponse::InternalServerError().body("Error Adding IP"),
    }
}
