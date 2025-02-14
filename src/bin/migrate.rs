#[tokio::main]
async fn main() {
    println!("Starting migration...");
    let pool = sqlx::postgres::PgPool::connect(
        "postgres://postgres:123456@localhost:3214/postgres?sslmode=disable",
    )
    .await
    .unwrap();

    println!("Connected to database, running migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations completed successfully!"),
        Err(e) => eprintln!("Migration error: {}", e),
    }
}
