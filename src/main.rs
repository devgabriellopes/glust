mod storage;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let pool = storage::db::Database::create_connection().await;
    println!("Successfully connected to the database!");
    // Execute a simple query
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await?;

    println!("Query result: {:?}", row.0);

    Ok(())
}
