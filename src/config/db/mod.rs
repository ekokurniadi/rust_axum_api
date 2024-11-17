use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn init() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
