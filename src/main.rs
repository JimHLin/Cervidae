use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
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
        .route("/", get(root))
        .route("/user/create", post(create_user))
        .route("/user", get(get_all_user))
        .route("/user/{:user_id}", get(get_user))
        .route("/user/update", patch(update_user))
        .route("/user/delete/{:user_id}", delete(delete_user))
        .route("/deer/create", post(create_deer))
        .route("/deer", get(get_all_deer))
        .route("/deer/{:deer_id}", get(get_deer))
        .route("/deer/update", patch(update_deer))
        .route("/deer/delete/{:deer_id}", delete(delete_deer))
        .route("/review/create", post(create_review))
        .route("/review/user/{:user_id}", get(get_review_by_user_id))
        .route("/review/deer/{:deer_id}", get(get_review_by_deer_id))
        .route("/review/update", patch(update_review))
        .route("/review/delete", delete(delete_review))
        .route("/comment/create", post(create_comment))
        .route("/comment/user/{:user_id}", get(get_comment_by_user_id))
        .route("/comment/deer/{:deer_id}", get(get_comment_by_deer_id))
        .route("/comment/update", patch(update_comment))
        .route("/comment/delete", delete(delete_comment))
        .route("/crime/create", post(create_crime))
        .route("/crime/all", get(get_all_crimes))
        .route("/crime/update", patch(update_crime))
        .route("/crime/delete/{:crime_id}", delete(delete_crime))
        .route("/background/asign", post(add_crime_to_deer))
        .route("/background/{:deer_id}", get(get_crime_by_deer_id))
        .route("/background/delete", delete(delete_crime_from_deer))
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
struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
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
struct Deer {
    id: Uuid,
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    kill_count: Option<i64>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    created_by: Uuid,
    updated_by: Uuid,
}

#[derive(Deserialize, Serialize)]
struct CreateDeerInput {
    user_id: Uuid,
    name: String,
    description: String,
    image_url: Option<String>,
    kill_count: Option<i64>,
}

#[derive(Deserialize, Serialize)]
struct UpdateDeerInput {
    user_id: Uuid,
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    kill_count: Option<i64>,
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
struct Review {
    user_id: Uuid,
    cervidae_id: Uuid,
    danger_level: i32,
    title: String,
    body: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
struct CreateReviewInput {
    user_id: Uuid,
    cervidae_id: Uuid,
    danger_level: i32,
    title: String,
    body: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateReviewInput {
    user_id: Uuid,
    cervidae_id: Uuid,
    danger_level: Option<i32>,
    title: Option<String>,
    body: Option<String>,
}

impl UpdateReviewInput {
    fn is_empty(&self) -> bool {
        self.danger_level.is_none() && self.title.is_none() && self.body.is_none()
    }
}

#[derive(Deserialize, Serialize)]
struct Comment {
    id: Uuid,
    user_id: Uuid,
    cervidae_id: Uuid,
    parent_id: Option<Uuid>,
    content: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
struct CreateCommentInput {
    user_id: Uuid,
    cervidae_id: Uuid,
    parent_id: Option<Uuid>,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateCommentInput {
    id: Uuid,
    content: Option<String>,
}

impl UpdateCommentInput {
    fn is_empty(&self) -> bool {
        self.content.is_none()
    }
}

#[derive(Deserialize, Serialize)]
struct Crime {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
struct CreateCrimeInput {
    name: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateCrimeInput {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
}

impl UpdateCrimeInput {
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none()
    }
}

#[derive(Deserialize, Serialize)]
struct CrimeCervidae {
    crime_id: Uuid,
    cervidae_id: Uuid,
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
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    if let Some(user) = user {
        Ok((StatusCode::OK, Json(user)))
    } else {
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    }
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

async fn delete_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Users WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "User deleted successfully".to_string()))
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

async fn delete_deer(
    State(pool): State<PgPool>,
    Path(deer_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Cervidae WHERE id = $1")
        .bind(deer_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "Deer deleted successfully".to_string()))
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
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    if let Some(deer) = deer {
        Ok((StatusCode::OK, Json(deer)))
    } else {
        Err((StatusCode::NOT_FOUND, "Deer not found".to_string()))
    }
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

async fn get_review_by_deer_id(
    State(pool): State<PgPool>,
    Path(deer_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Review>>), (StatusCode, String)> {
    let reviews = sqlx::query_as!(
        Review,
        "SELECT * FROM Review WHERE cervidae_id = $1",
        deer_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;
    Ok((StatusCode::OK, Json(reviews)))
}

async fn get_review_by_user_id(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Review>>), (StatusCode, String)> {
    let reviews = sqlx::query_as!(Review, "SELECT * FROM Review WHERE user_id = $1", user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(reviews)))
}

async fn create_review(
    State(pool): State<PgPool>,
    Json(review): Json<CreateReviewInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let review_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO Review (user_id, cervidae_id, danger_level, title, body)
         VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(&review.user_id)
    .bind(&review.cervidae_id)
    .bind(&review.danger_level)
    .bind(&review.title)
    .bind(&review.body)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, review_id.to_string()))
}

async fn update_review(
    State(pool): State<PgPool>,
    Json(review): Json<UpdateReviewInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if review.is_empty() {
        return Ok((
            StatusCode::EXPECTATION_FAILED,
            "No valid fields to update".to_string(),
        ));
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE Review SET updated_at = NOW()");

    if let Some(danger_level) = &review.danger_level {
        add_to_query(&mut query_builder, "danger_level", danger_level);
    }
    if let Some(title) = &review.title {
        add_to_query(&mut query_builder, "title", title);
    }
    if let Some(body) = &review.body {
        add_to_query(&mut query_builder, "body", body);
    }

    query_builder
        .push(" WHERE user_id = ")
        .push_bind(&review.user_id);
    query_builder
        .push(" AND cervidae_id = ")
        .push_bind(&review.cervidae_id);
    let query = query_builder.build();
    query.execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "Review updated successfully".to_string()))
}

async fn delete_review(
    State(pool): State<PgPool>,
    Json(review): Json<UpdateReviewInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Review WHERE user_id = $1 AND cervidae_id = $2")
        .bind(&review.user_id)
        .bind(&review.cervidae_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "Review deleted successfully".to_string()))
}

async fn get_comment_by_deer_id(
    State(pool): State<PgPool>,
    Path(deer_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Comment>>), (StatusCode, String)> {
    let comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM Comment WHERE cervidae_id = $1",
        deer_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;
    Ok((StatusCode::OK, Json(comments)))
}

async fn get_comment_by_user_id(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Comment>>), (StatusCode, String)> {
    let comments = sqlx::query_as!(Comment, "SELECT * FROM Comment WHERE user_id = $1", user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(comments)))
}

async fn get_comment(
    State(pool): State<PgPool>,
    Path(comment_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Comment>), (StatusCode, String)> {
    let comment = sqlx::query_as!(Comment, "SELECT * FROM Comment WHERE id = $1", comment_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    if let Some(comment) = comment {
        Ok((StatusCode::OK, Json(comment)))
    } else {
        Err((StatusCode::NOT_FOUND, "Comment not found".to_string()))
    }
}

async fn create_comment(
    State(pool): State<PgPool>,
    Json(comment): Json<CreateCommentInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let comment_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO Comment (id, user_id, cervidae_id, parent_id, content)
         VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(comment_id)
    .bind(&comment.user_id)
    .bind(&comment.cervidae_id)
    .bind(&comment.parent_id)
    .bind(&comment.content)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, comment_id.to_string()))
}

async fn update_comment(
    State(pool): State<PgPool>,
    Json(comment): Json<UpdateCommentInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if comment.is_empty() {
        return Ok((
            StatusCode::EXPECTATION_FAILED,
            "No valid fields to update".to_string(),
        ));
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE Comment SET updated_at = NOW()");

    if let Some(content) = &comment.content {
        add_to_query(&mut query_builder, "content", content);
    }

    query_builder.push(" WHERE id = ").push_bind(comment.id);
    let query = query_builder.build();
    query.execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "Comment updated successfully".to_string()))
}

async fn delete_comment(
    State(pool): State<PgPool>,
    Path(comment_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Comment WHERE id = $1")
        .bind(comment_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "Comment deleted successfully".to_string()))
}

async fn create_crime(
    State(pool): State<PgPool>,
    Json(crime): Json<CreateCrimeInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let crime_id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO Crime (id, name, description)
         VALUES ($1, $2, $3)"#,
    )
    .bind(crime_id)
    .bind(&crime.name)
    .bind(&crime.description)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, crime_id.to_string()))
}

async fn get_all_crimes(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Crime>>), (StatusCode, String)> {
    let crimes = sqlx::query_as!(Crime, "SELECT * FROM Crime")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(crimes)))
}

async fn update_crime(
    State(pool): State<PgPool>,
    Json(crime): Json<UpdateCrimeInput>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    if crime.is_empty() {
        return Ok((
            StatusCode::EXPECTATION_FAILED,
            "No valid fields to update".to_string(),
        ));
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE Crime SET updated_at = NOW()");

    if let Some(name) = &crime.name {
        add_to_query(&mut query_builder, "name", name);
    }
    if let Some(description) = &crime.description {
        add_to_query(&mut query_builder, "description", description);
    }

    query_builder.push(" WHERE id = ").push_bind(crime.id);
    let query = query_builder.build();
    query.execute(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "Crime updated successfully".to_string()))
}

async fn delete_crime(
    State(pool): State<PgPool>,
    Path(crime_id): Path<Uuid>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Crime WHERE id = $1")
        .bind(crime_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "Crime deleted successfully".to_string()))
}

async fn add_crime_to_deer(
    State(pool): State<PgPool>,
    Json(crime_cervidae): Json<CrimeCervidae>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query(
        r#"
        INSERT INTO Crime_Cervidae (crime_id, cervidae_id)
         VALUES ($1, $2)"#,
    )
    .bind(crime_cervidae.crime_id)
    .bind(crime_cervidae.cervidae_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, crime_cervidae.crime_id.to_string()))
}

async fn get_crime_by_deer_id(
    State(pool): State<PgPool>,
    Path(deer_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Crime>>), (StatusCode, String)> {
    let crimes = sqlx::query_as!(Crime, "SELECT * FROM Crime WHERE id IN (SELECT crime_id FROM Crime_Cervidae WHERE cervidae_id = $1)", deer_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    Ok((StatusCode::OK, Json(crimes)))
}

async fn delete_crime_from_deer(
    State(pool): State<PgPool>,
    Json(crime_cervidae): Json<CrimeCervidae>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM Crime_Cervidae WHERE crime_id = $1 AND cervidae_id = $2")
        .bind(crime_cervidae.crime_id)
        .bind(crime_cervidae.cervidae_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok((
        StatusCode::OK,
        "Crime deleted from deer successfully".to_string(),
    ))
}
