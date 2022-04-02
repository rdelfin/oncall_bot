use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn initialise() -> sqlx::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let _row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    Ok(pool)
}
