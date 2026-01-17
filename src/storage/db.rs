use std::env;

use sqlx::Postgres;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;

pub struct Database;

impl Database {
    pub async fn create_connection () -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL")
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.unwrap();
    return pool
}
}

