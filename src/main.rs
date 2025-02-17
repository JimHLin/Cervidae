use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use chrono::NaiveDateTime;
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
        .route("/user/create", post(create_user))
        .route("/user", get(get_all_user))
        .route("/user/{:user_id}", get(get_user))
        .route("/user/update", patch(update_user))
        .route("/deer/create", post(create_deer))
        .route("/deer", get(get_all_deer))
        .route("/deer/{:deer_id}", get(get_deer))
        .route("/deer/update", patch(update_deer))
        .with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Deer>), (StatusCode, String)> {
    let deer = sqlx::query_as!(
        Deer,
        "SELECT * FROM Cervidae WHERE kill_count IN (SELECT MAX(kill_count) FROM Cervidae)"
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;
    if let Some(deer) = deer {
        return Ok((StatusCode::OK, Json(deer)));
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "They're all dead. You are next.".to_string(),
        ));
    }
}

#[derive(Deserialize, Serialize)]
struct CreateUserInput {
    //id: Uuid,
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateUserInput {
    id: Uuid,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl UpdateUserInput {
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.email.is_none() && self.password.is_none()
    }
}

#[derive(Deserialize, Serialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
struct Deer {
    id: Uuid,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    kill_count: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    created_by: Uuid,
    updated_by: Uuid,
}

#[derive(Deserialize, Serialize)]
struct UpdateDeerInput {
    user_id: Uuid,
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    kill_count: Option<i32>,
}

impl UpdateDeerInput {
    fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.image_url.is_none()
            && self.kill_count.is_none()
    }
}

#[derive(Deserialize, Serialize)]
struct CreateDeerInput {
    user_id: Uuid,
    name: String,
    description: String,
    image_url: Option<String>,
    kill_count: Option<i32>,
}

struct DeerError(StatusCode, String);

fn add_to_query<'b, 'a, T>(
    query_builder: &'b mut sqlx::QueryBuilder<'a, sqlx::Postgres>,
    key: &str,
    value: &'a T,
) where
    T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a,
{
    query_builder.push(", ");
    query_builder.push(key);
    query_builder.push(" = ");
    query_builder.push_bind(value);
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUserInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO Users (id, name, email, password)
         VALUES ($1, $2, $3, $4)"#,
    )
    .bind(user_id)
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, user_id.to_string()))
}

async fn update_user(
    State(pool): State<PgPool>,
    Json(user): Json<UpdateUserInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if user.is_empty() {
        return Err((
            StatusCode::EXPECTATION_FAILED,
            "No valid fields to update".to_string(),
        ));
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE Users SET updated_at = NOW()");

    if let Some(name) = &user.name {
        add_to_query(&mut query_builder, "name", name);
    }

    if let Some(email) = &user.email {
        add_to_query(&mut query_builder, "email", email);
    }

    if let Some(password) = &user.password {
        add_to_query(&mut query_builder, "password", password);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(user.id);
    let query = query_builder.build();
    query.execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "User updated successfully".to_string()))
}

async fn get_all_user(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, String)> {
    let users = sqlx::query_as!(User, "SELECT * FROM Users")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, Json(users)))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = sqlx::query_as!(User, "SELECT * FROM Users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, Json(user)))
}

async fn create_deer(
    State(pool): State<PgPool>,
    Json(deer): Json<CreateDeerInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let deer_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO Cervidae (id, name, description, created_by, updated_by, image_url, kill_count)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(deer_id)
    .bind(&deer.name)
    .bind(&deer.description)
    .bind(&deer.user_id)
    .bind(&deer.user_id)
    .bind(&deer.image_url)
    .bind(&deer.kill_count)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, deer_id.to_string()))
}

async fn get_all_deer(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Deer>>), (StatusCode, String)> {
    let deer = sqlx::query_as!(Deer, "SELECT * FROM Cervidae")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(deer)))
}

async fn get_deer(
    State(pool): State<PgPool>,
    Path(deer_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Deer>), (StatusCode, String)> {
    let deer = sqlx::query_as!(Deer, "SELECT * FROM Cervidae WHERE id = $1", deer_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(deer)))
}

async fn update_deer(
    State(pool): State<PgPool>,
    Json(deer): Json<UpdateDeerInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if deer.is_empty() {
        return Ok((
            StatusCode::EXPECTATION_FAILED,
            "No valid fields to update".to_string(),
        ));
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE Cervidae SET updated_at = NOW()");

    add_to_query(&mut query_builder, "updated_by", &deer.user_id);

    if let Some(name) = &deer.name {
        add_to_query(&mut query_builder, "name", name);
    }
    if let Some(description) = &deer.description {
        add_to_query(&mut query_builder, "description", description);
    }
    if let Some(image_url) = &deer.image_url {
        add_to_query(&mut query_builder, "image_url", image_url);
    }
    if let Some(kill_count) = &deer.kill_count {
        add_to_query(&mut query_builder, "kill_count", kill_count);
    }

    query_builder.push(" WHERE id = ").push_bind(deer.id);
    let query = query_builder.build();
    query.execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "Deer updated successfully".to_string()))
}
