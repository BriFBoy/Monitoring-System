use std::process::{self, exit};

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    web::{self, Data},
};
use dotenv::dotenv;
use monitoring_backend_rs::AppData;
use monitoring_backend_rs::api::delete_ip;
use monitoring_backend_rs::api::{add_ip, get_ips, ping, sys_info, sys_metric};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let port: u16 = 8081;
    let sock = format!("0.0.0.0:{port}");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        println!("No DATABASE_URL enviroment var set. Shuting down");
        process::exit(1);
    });

    // Sets up a connection pool. Exits if it is unable to do so.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| {
            println!("Error getting connection. Shuting down");
            process::exit(1);
        });

    // Runs all the sql files on start up
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migration");

    println!("Starting server on {sock}");

    // Sets up the HttpServer and starts it
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        // Creates all the API endpoints
        App::new()
            .wrap(cors)
            .app_data(Data::new(AppData { db: pool.clone() }))
            .service(
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
    .bind(&sock)
    .unwrap()
    .run()
    .await
    .unwrap_or_else(|err| {
        println!("Unexpected shutdown! Error: {}", err);
        exit(1);
    });
}
