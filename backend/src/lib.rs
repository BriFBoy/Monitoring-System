use sqlx::{Pool, Postgres};

/// All the logic for comunication with the agent
pub mod agent;

/// all API endpoints
pub mod api;

/// All the logic for comunication with the postgres database
pub mod database;
pub mod systeminfo;
pub mod systemmetrics;

/// The struct to store the Database connection pool for postgres
pub struct AppData {
    pub db: Pool<Postgres>,
}
