use async_graphql::{Context, Object, Result};
use bcrypt::{hash, verify};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use models::*;
use sqlx::{self, query, query_as, Encode, PgPool, Postgres, QueryBuilder, Type};
use std::env;
use uuid::Uuid;

pub mod models;
pub mod storage;
// Root types for GraphQL schema
pub struct QueryRoot;

fn add_to_query<'b, 'a, T>(
    query_builder: &'b mut QueryBuilder<'a, Postgres>,
    key: &str,
    value: &'a T,
) where
    T: Encode<'a, Postgres> + Type<Postgres> + 'a,
{
    query_builder.push(", ");
    query_builder.push(key);
    query_builder.push(" = ");
    query_builder.push_bind(value);
}

#[Object]
impl QueryRoot {
    // Add your query resolvers here
    async fn users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let users = query_as!(User, "SELECT * FROM Users")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;

        Ok(users)
    }

    async fn user(&self, context: &Context<'_>, id: UuidScalar) -> Result<Option<User>> {
        let id: Uuid = id.into();
        let user = query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn deer(&self, context: &Context<'_>, id: UuidScalar) -> Result<Option<Deer>> {
        let id: Uuid = id.into();
        let deer = query_as!(Deer, "SELECT * FROM Cervidae WHERE id = $1", id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await?;

        Ok(deer)
    }

    async fn deer_all(&self, context: &Context<'_>) -> Result<Vec<Deer>> {
        let deer = query_as!(Deer, "SELECT * FROM Cervidae")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;

        Ok(deer)
    }

    async fn deer_reviews(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Review>> {
        let id: Uuid = id.into();
        let reviews = query_as!(Review, "SELECT * FROM review WHERE cervidae_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(reviews)
    }

    async fn user_reviews(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Review>> {
        let id: Uuid = id.into();
        let reviews = query_as!(Review, "SELECT * FROM review WHERE user_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(reviews)
    }

    async fn deer_comments(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Comment>> {
        let id: Uuid = id.into();
        let comments = query_as!(
            Comment,
            "SELECT * FROM comment WHERE cervidae_id = $1 ORDER BY created_at DESC",
            id
        )
        .fetch_all(context.data_unchecked::<PgPool>())
        .await?;
        Ok(comments)
    }

    async fn user_comments(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Comment>> {
        let id: Uuid = id.into();
        let comments = query_as!(Comment, "SELECT * FROM comment WHERE user_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(comments)
    }

    async fn crimes(&self, context: &Context<'_>) -> Result<Vec<Crime>> {
        let crimes = query_as!(Crime, "SELECT * FROM crime")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(crimes)
    }

    async fn deer_crimes(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Crime>> {
        let id: Uuid = id.into();
        let crimes = query_as!(Crime, "SELECT * FROM Crime WHERE id IN (SELECT crime_id FROM Crime_Cervidae WHERE cervidae_id = $1)", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(crimes)
    }

    async fn crime_deer(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<Deer>> {
        let id: Uuid = id.into();
        let deer = query_as!(Deer, "SELECT * FROM Cervidae WHERE id IN (SELECT cervidae_id FROM Crime_Cervidae WHERE crime_id = $1)", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(deer)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Add your mutation resolvers here
    async fn create_user(&self, context: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let user_id = uuid::Uuid::new_v4();
        let hashed = hash(input.password, 10)?;
        let user = query_as!(
            User,
            r#"
            INSERT INTO Users (id, name, email, password)
             VALUES ($1, $2, $3, $4) RETURNING *"#,
            user_id,
            &input.name,
            &input.email,
            hashed,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(user)
    }

    async fn update_user(&self, context: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        if input.is_empty() {
            return Err(async_graphql::Error::new_with_source(
                "No update fields provided",
            ));
        }
        let user_id = Uuid::from(input.id);
        let mut query = sqlx::QueryBuilder::new("UPDATE Users SET updated_at = NOW()");
        if let Some(name) = &input.name {
            add_to_query(&mut query, "name", name);
        }
        if let Some(email) = &input.email {
            add_to_query(&mut query, "email", email);
        }
        if let Some(password) = &input.password {
            add_to_query(&mut query, "password", password);
        }
        query.push(" WHERE id = ");
        query.push_bind(user_id);
        query.push(" RETURNING *;");
        let user: User = query
            .build_query_as()
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        Ok(user)
    }

    async fn delete_user(&self, context: &Context<'_>, id: UuidScalar) -> Result<String> {
        let id: Uuid = id.into();
        let result = query!("DELETE FROM Users WHERE id = $1", id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;

        match result.rows_affected() {
            0 => Err("User not found".into()),
            _ => Ok("User deleted successfully".to_string()),
        }
    }

    async fn create_deer(&self, context: &Context<'_>, input: CreateDeerInput) -> Result<Deer> {
        let deer_id = uuid::Uuid::new_v4();
        let user_id: Uuid = input.user_id.into();
        let deer = query_as!(
            Deer,
            r#"
            INSERT INTO Cervidae (id, name, description, image_url, kill_count, created_by, updated_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
            deer_id,
            &input.name,
            &input.description,
            input.image_url,
            input.kill_count,
            user_id,
            user_id,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(deer.into())
    }

    async fn update_deer(&self, context: &Context<'_>, input: UpdateDeerInput) -> Result<Deer> {
        if input.is_empty() {
            return Err(async_graphql::Error::new_with_source(
                "No update fields provided",
            ));
        }
        let deer_id = Uuid::from(input.id);
        let user_id: Uuid = input.user_id.into();
        let mut query = QueryBuilder::new("UPDATE Cervidae SET updated_at = NOW(), updated_by = ");
        query.push_bind(user_id);
        if let Some(name) = &input.name {
            add_to_query(&mut query, "name", name);
        }
        if let Some(description) = &input.description {
            add_to_query(&mut query, "description", description);
        }
        if let Some(kill_count) = &input.kill_count {
            add_to_query(&mut query, "kill_count", kill_count);
        }
        if let Some(image_url) = &input.image_url {
            add_to_query(&mut query, "image_url", image_url);
        }
        query.push(" WHERE id = ");
        query.push_bind(deer_id);
        query.push(" RETURNING *;");
        let deer: Deer = query
            .build_query_as()
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        Ok(deer.into())
    }

    async fn delete_deer(&self, context: &Context<'_>, id: UuidScalar) -> Result<String> {
        let id: Uuid = id.into();
        let result = query("DELETE FROM Cervidae WHERE id = $1")
            .bind(id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        match result.rows_affected() {
            0 => Err("Deer not found".into()),
            _ => Ok("Deer deleted successfully".to_string()),
        }
    }

    async fn create_review(
        &self,
        context: &Context<'_>,
        input: CreateReviewInput,
    ) -> Result<Review> {
        let user_id: Uuid = input.user_id.into();
        let cervidae_id: Uuid = input.cervidae_id.into();
        let review = query_as!(
            Review,
            r#"
            INSERT INTO review (user_id, cervidae_id, danger_level, title, body)
             VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            user_id,
            cervidae_id,
            &input.danger_level,
            &input.title,
            &input.body,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(review)
    }

    async fn update_review(
        &self,
        context: &Context<'_>,
        input: UpdateReviewInput,
    ) -> Result<Review> {
        if input.is_empty() {
            return Err(async_graphql::Error::new_with_source(
                "No update fields provided",
            ));
        }
        let user_id: Uuid = input.user_id.into();
        let cervidae_id: Uuid = input.cervidae_id.into();
        let mut query = QueryBuilder::new("UPDATE review SET updated_at = NOW()");
        if let Some(danger_level) = &input.danger_level {
            add_to_query(&mut query, "danger_level", danger_level);
        }
        if let Some(title) = &input.title {
            add_to_query(&mut query, "title", title);
        }
        if let Some(body) = &input.body {
            add_to_query(&mut query, "body", body);
        }
        query.push(" WHERE user_id = ");
        query.push_bind(user_id);
        query.push(" AND cervidae_id = ");
        query.push_bind(cervidae_id);
        query.push(" RETURNING *;");

        let review: Review = query
            .build_query_as()
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        Ok(review)
    }

    async fn delete_review(
        &self,
        context: &Context<'_>,
        input: UpdateReviewInput,
    ) -> Result<String> {
        let user_id = Uuid::from(input.user_id);
        let cervidae_id = Uuid::from(input.cervidae_id);
        let result = query("DELETE FROM review WHERE user_id = $1 AND cervidae_id = $2")
            .bind(user_id)
            .bind(cervidae_id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        match result.rows_affected() {
            0 => Err("Review not found".into()),
            _ => Ok("Review deleted successfully".to_string()),
        }
    }

    async fn create_comment(
        &self,
        context: &Context<'_>,
        input: CreateCommentInput,
    ) -> Result<Comment> {
        let comment_id = uuid::Uuid::new_v4();
        let user_id: Uuid = input.user_id.into();
        let cervidae_id: Uuid = input.cervidae_id.into();
        let parent_id: Option<Uuid> = input.parent_id.map(|id| id.into());
        let comment = query_as!(
            Comment,
            r#"
            INSERT INTO comment (id, user_id, cervidae_id, parent_id, content)
             VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            comment_id,
            user_id,
            cervidae_id,
            parent_id,
            &input.content,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(comment)
    }

    async fn update_comment(
        &self,
        context: &Context<'_>,
        input: UpdateCommentInput,
    ) -> Result<Comment> {
        if input.is_empty() {
            return Err(async_graphql::Error::new_with_source(
                "No update fields provided",
            ));
        }
        let comment_id = Uuid::from(input.id);
        let mut query = QueryBuilder::new("UPDATE comment SET updated_at = NOW()");
        if let Some(content) = &input.content {
            add_to_query(&mut query, "content", content);
        }
        query.push(" WHERE id = ");
        query.push_bind(comment_id);
        query.push(" RETURNING *;");

        let comment: Comment = query
            .build_query_as()
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        Ok(comment.into())
    }

    async fn delete_comment(&self, context: &Context<'_>, id: UuidScalar) -> Result<String> {
        let id: Uuid = id.into();
        let result = query("DELETE FROM comment WHERE id = $1")
            .bind(id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        match result.rows_affected() {
            0 => Err("Comment not found".into()),
            _ => Ok("Comment deleted successfully".to_string()),
        }
    }

    async fn create_crime(&self, context: &Context<'_>, input: CreateCrimeInput) -> Result<Crime> {
        let crime_id = uuid::Uuid::new_v4();
        let crime = query_as!(
            Crime,
            r#"
            INSERT INTO crime (id, name, description)
             VALUES ($1, $2, $3) RETURNING *"#,
            crime_id,
            &input.name,
            &input.description,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(crime.into())
    }

    async fn update_crime(&self, context: &Context<'_>, input: UpdateCrimeInput) -> Result<Crime> {
        if input.is_empty() {
            return Err(async_graphql::Error::new_with_source(
                "No update fields provided",
            ));
        }
        let crime_id = Uuid::from(input.id);
        let mut query = QueryBuilder::new("UPDATE crime SET updated_at = NOW()");
        if let Some(name) = &input.name {
            add_to_query(&mut query, "name", name);
        }
        if let Some(description) = &input.description {
            add_to_query(&mut query, "description", description);
        }
        query.push(" WHERE id = ");
        query.push_bind(crime_id);
        query.push(" RETURNING *;");

        let crime: Crime = query
            .build_query_as()
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        Ok(crime.into())
    }

    async fn delete_crime(&self, context: &Context<'_>, id: UuidScalar) -> Result<String> {
        let id: Uuid = id.into();
        let result = query("DELETE FROM crime WHERE id = $1")
            .bind(id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        match result.rows_affected() {
            0 => Err("Crime not found".into()),
            _ => Ok("Crime deleted successfully".to_string()),
        }
    }

    async fn assign_crime(
        &self,
        context: &Context<'_>,
        input: CrimeCervidaeInput,
    ) -> Result<String> {
        let crime_id: Uuid = input.crime_id.into();
        let cervidae_id: Uuid = input.cervidae_id.into();
        let crime_cervidae = query_as!(
            CrimeCervidae,
            r#"
            INSERT INTO crime_cervidae (crime_id, cervidae_id)
             VALUES ($1, $2) RETURNING *"#,
            crime_id,
            cervidae_id,
        )
        .fetch_one(context.data_unchecked::<PgPool>())
        .await?;
        Ok(format!(
            "Crime {} assigned to deer {}",
            crime_cervidae.crime_id, crime_cervidae.cervidae_id
        ))
    }

    async fn drop_crime(&self, context: &Context<'_>, input: CrimeCervidaeInput) -> Result<String> {
        let crime_id: Uuid = input.crime_id.into();
        let cervidae_id: Uuid = input.cervidae_id.into();
        let result = query("DELETE FROM crime_cervidae WHERE crime_id = $1 AND cervidae_id = $2")
            .bind(crime_id)
            .bind(cervidae_id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        match result.rows_affected() {
            0 => Err("Crime assignment not found".into()),
            _ => Ok("Crime dropped successfully".to_string()),
        }
    }

    async fn login(&self, context: &Context<'_>, input: LoginInput) -> Result<String> {
        let user = query_as!(User, "SELECT * FROM Users WHERE email = $1", input.email)
            .fetch_one(context.data_unchecked::<PgPool>())
            .await?;
        let password_match = verify(input.password, &user.password).unwrap();
        if password_match {
            let _ = query("UPDATE Users SET last_login = NOW() WHERE id = $1")
                .bind(user.id)
                .execute(context.data_unchecked::<PgPool>())
                .await?;
            let header = Header::default();
            let claims = Claims {
                sub: user.id.to_string(),
                exp: (Utc::now().timestamp() + 3600) as usize,
                iat: Utc::now().timestamp() as usize,
                iss: "National Cervidae Analystics Association".to_string(),
                is_admin: user.is_admin,
            };
            let key = EncodingKey::from_secret(env::var("CLIENT_SECRET")?.as_bytes());
            let token = encode(&header, &claims, &key)?;
            return Ok(token);
        } else {
            return Err("Login failed".into());
        }
    }
}
