use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // setup database connection
    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/deer", post(create_deer))
        .with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Serialize)]
struct Deer {
    name: String,
    description: String,
}

struct DeerError(StatusCode, String);

async fn create_deer(
    State(pool): State<PgPool>,
    Json(deer): Json<Deer>,
) -> Result<(StatusCode, Json<Deer>), (StatusCode, String)> {
    sqlx::query(
        r#"
        INSERT INTO Cervidae (id, name, description)
         VALUES ($1, $2, $3)"#,
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&deer.name)
    .bind(&deer.description)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, Json(deer)))
}

// the input to our `create_user` handler
