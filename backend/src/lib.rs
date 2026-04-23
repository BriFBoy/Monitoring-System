use sqlx::{Pool, Postgres};

pub mod agent;
pub mod api;
pub mod database;
pub mod systeminfo;
pub mod systemmetrics;

pub struct AppData {
    pub db: Pool<Postgres>,
}
