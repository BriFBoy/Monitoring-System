use sqlx::{FromRow, Pool, Postgres};

#[derive(serde::Serialize, FromRow)]
pub struct IPaddr {
    ip: String,
    port: i16,
}

impl IPaddr {
    pub async fn get_ipaddr_from_ip(
        db: &Pool<Postgres>,
        ip: &str,
    ) -> Result<IPaddr, sqlx::error::Error> {
        sqlx::query_as::<_, IPaddr>("SELECT * FROM ipaddr WHERE ip = $1")
            .bind(ip)
            .fetch_one(db)
            .await
    }
}
