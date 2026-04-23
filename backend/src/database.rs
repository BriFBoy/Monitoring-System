use sqlx::{FromRow, Pool, Postgres, postgres::PgQueryResult};

/// The struct used for storing IPs and ports in the database
#[derive(serde::Serialize, FromRow, serde::Deserialize)]
pub struct IPaddr {
    pub ip: String,
    pub port: i32,
}

impl IPaddr {
    pub fn new(ip: String, port: i32) -> IPaddr {
        IPaddr { ip, port }
    }

    /// Gets the ip form the database using a ip.
    /// Used to check if that ip is stored in the databse or to get the port of the specified ip
    pub async fn get_ipaddr_from_ip(
        db: &Pool<Postgres>,
        ip: &str,
    ) -> Result<IPaddr, sqlx::error::Error> {
        sqlx::query_as::<_, IPaddr>("SELECT * FROM ipaddr WHERE ip = $1")
            .bind(ip)
            .fetch_one(db)
            .await
    }

    /// Gets all the Ips form the database
    /// # Retuns
    /// Returns a Vector of all the ips. If any errors occurs the method wil return a empty Vec
    pub async fn get_all_ips(db: &Pool<Postgres>) -> Vec<IPaddr> {
        sqlx::query_as!(IPaddr, "SELECT ip, port FROM ipaddr")
            .fetch_all(db)
            .await
            .unwrap_or(Vec::new())
    }

    /// Inserts the given IPaddr to the database
    pub async fn insert_ipaddr(&self, db: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO ipaddr (ip, port) VALUES ($1, $2)",
            self.ip,
            self.port
        )
        .execute(db)
        .await
    }

    /// Deletes the given IPaddr from database
    pub async fn delete_ip(&self, db: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM ipaddr WHERE ip = $1", self.ip)
            .execute(db)
            .await
    }
}
