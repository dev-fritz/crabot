use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set").as_str())
        .await?;
    Ok(pool)
}