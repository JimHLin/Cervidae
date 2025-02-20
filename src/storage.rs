use crate::models::*;
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

// Root types for GraphQL schema
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // Add your query resolvers here
    async fn users(&self, context: &Context<'_>) -> Result<Vec<UserOutput>> {
        let users = sqlx::query_as!(User, "SELECT * FROM Users")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;

        Ok(users.into_iter().map(UserOutput::from).collect())
    }

    async fn user(&self, context: &Context<'_>, id: UuidScalar) -> Result<Option<UserOutput>> {
        let id: Uuid = id.into();
        let user = sqlx::query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await
            .map_err(|e| e.to_string())?;

        Ok(user.map(UserOutput::from))
    }

    async fn deer(&self, context: &Context<'_>, id: UuidScalar) -> Result<Option<DeerOutput>> {
        let id: Uuid = id.into();
        let deer = sqlx::query_as!(Deer, "SELECT * FROM Cervidae WHERE id = $1", id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await?;

        Ok(deer.map(DeerOutput::from))
    }

    async fn deer_all(&self, context: &Context<'_>) -> Result<Vec<DeerOutput>> {
        let deer = sqlx::query_as!(Deer, "SELECT * FROM Cervidae")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;

        Ok(deer.into_iter().map(DeerOutput::from).collect())
    }

    async fn deer_reviews(
        &self,
        context: &Context<'_>,
        id: UuidScalar,
    ) -> Result<Vec<ReviewOutput>> {
        let id: Uuid = id.into();
        let reviews = sqlx::query_as!(Review, "SELECT * FROM review WHERE cervidae_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(reviews.into_iter().map(ReviewOutput::from).collect())
    }

    async fn user_reviews(
        &self,
        context: &Context<'_>,
        id: UuidScalar,
    ) -> Result<Vec<ReviewOutput>> {
        let id: Uuid = id.into();
        let reviews = sqlx::query_as!(Review, "SELECT * FROM review WHERE user_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(reviews.into_iter().map(ReviewOutput::from).collect())
    }

    async fn deer_comments(
        &self,
        context: &Context<'_>,
        id: UuidScalar,
    ) -> Result<Vec<CommentOutput>> {
        let id: Uuid = id.into();
        let comments = sqlx::query_as!(Comment, "SELECT * FROM comment WHERE cervidae_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(comments.into_iter().map(CommentOutput::from).collect())
    }

    async fn user_comments(
        &self,
        context: &Context<'_>,
        id: UuidScalar,
    ) -> Result<Vec<CommentOutput>> {
        let id: Uuid = id.into();
        let comments = sqlx::query_as!(Comment, "SELECT * FROM comment WHERE user_id = $1", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(comments.into_iter().map(CommentOutput::from).collect())
    }

    async fn crimes(&self, context: &Context<'_>) -> Result<Vec<CrimeOutput>> {
        let crimes = sqlx::query_as!(Crime, "SELECT * FROM crime")
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(crimes.into_iter().map(CrimeOutput::from).collect())
    }

    async fn deer_crimes(&self, context: &Context<'_>, id: UuidScalar) -> Result<Vec<CrimeOutput>> {
        let id: Uuid = id.into();
        let crimes = sqlx::query_as!(Crime, "SELECT * FROM Crime WHERE id IN (SELECT crime_id FROM Crime_Cervidae WHERE cervidae_id = $1)", id)
            .fetch_all(context.data_unchecked::<PgPool>())
            .await?;
        Ok(crimes.into_iter().map(CrimeOutput::from).collect())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // Add your mutation resolvers here
    async fn create_user(
        &self,
        context: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<UserOutput> {
        let user_id = uuid::Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO Users (id, name, email, password)
             VALUES ($1, $2, $3, $4)"#,
        )
        .bind(user_id)
        .bind(&input.name)
        .bind(&input.email)
        .bind(&input.password)
        .execute(context.data_unchecked::<PgPool>())
        .await?;
        let user = sqlx::query_as!(User, "SELECT * FROM Users WHERE id = $1", user_id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await
            .map_err(|e| e.to_string())?;
        if let Some(user) = user {
            Ok(user.into())
        } else {
            Err(async_graphql::Error::new_with_source(
                "User creation failed",
            ))
        }
    }

    async fn update_user(
        &self,
        context: &Context<'_>,
        input: UpdateUserInput,
    ) -> Result<UserOutput> {
        let user_id = Uuid::from(input.id);
        let mut query = sqlx::QueryBuilder::new("UPDATE Users SET updated_at = NOW()");
        if let Some(name) = input.name {
            query.push(", name = ");
            query.push_bind(name);
        }
        if let Some(email) = input.email {
            query.push(", email = ");
            query.push_bind(email);
        }
        if let Some(password) = input.password {
            query.push(", password = ");
            query.push_bind(password);
        }
        query.push(" WHERE id = ");
        query.push_bind(user_id);
        query.push(";");

        let query = query.build();

        query.execute(context.data_unchecked::<PgPool>()).await?;

        let user = sqlx::query_as!(User, "SELECT * FROM Users WHERE id = $1", user_id)
            .fetch_optional(context.data_unchecked::<PgPool>())
            .await?;
        if let Some(user) = user {
            Ok(user.into())
        } else {
            Err(async_graphql::Error::new_with_source("User update failed"))
        }
    }

    async fn delete_user(&self, context: &Context<'_>, id: UuidScalar) -> Result<String> {
        let id: Uuid = id.into();
        sqlx::query("DELETE FROM Users WHERE id = $1")
            .bind(id)
            .execute(context.data_unchecked::<PgPool>())
            .await?;
        Ok("User deleted successfully".to_string())
    }
}
