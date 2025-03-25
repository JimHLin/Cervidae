use crate::graphql::models::*;
use async_graphql::{Context, Result};
use sqlx::{self, query_as, PgPool};
use uuid::Uuid;

pub async fn get_user(context: &Context<'_>, id: Uuid) -> Result<Option<User>> {
    let user = query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(user)
}

pub async fn get_deer(context: &Context<'_>, id: Uuid) -> Result<Option<Deer>> {
    let deer: Option<Deer> = query_as("SELECT * FROM Cervidae WHERE id = $1")
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(deer)
}

pub async fn get_comment(context: &Context<'_>, id: Uuid) -> Result<Option<Comment>> {
    let comment = query_as!(Comment, "SELECT * FROM Comment WHERE id = $1", id)
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(comment)
}

pub async fn get_reviews_by_deer(context: &Context<'_>, id: Uuid) -> Result<Vec<Review>> {
    let reviews = query_as!(Review, "SELECT * FROM Review WHERE cervidae_id = $1", id)
        .fetch_all(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(reviews)
}

pub async fn get_reviews_by_user(context: &Context<'_>, id: Uuid) -> Result<Vec<Review>> {
    let reviews = query_as!(Review, "SELECT * FROM Review WHERE user_id = $1", id)
        .fetch_all(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(reviews)
}

pub async fn get_comments_by_deer(context: &Context<'_>, id: Uuid) -> Result<Vec<Comment>> {
    let comments = query_as!(Comment, "SELECT * FROM Comment WHERE cervidae_id = $1", id)
        .fetch_all(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(comments)
}

pub async fn get_comments_by_user(context: &Context<'_>, id: Uuid) -> Result<Vec<Comment>> {
    let comments = query_as!(Comment, "SELECT * FROM Comment WHERE user_id = $1", id)
        .fetch_all(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(comments)
}

pub async fn get_crime(context: &Context<'_>, id: Uuid) -> Result<Option<Crime>> {
    let crime = query_as!(Crime, "SELECT * FROM Crime WHERE id = $1", id)
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(crime)
}

pub async fn get_crimes_by_deer(context: &Context<'_>, id: Uuid) -> Result<Vec<Crime>> {
    let crimes = query_as!(Crime, "SELECT * FROM Crime WHERE id IN (SELECT crime_id FROM Crime_Cervidae WHERE cervidae_id = $1)", id)
        .fetch_all(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(crimes)
}
