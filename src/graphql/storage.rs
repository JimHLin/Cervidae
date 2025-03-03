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
    let deer = query_as!(Deer, "SELECT * FROM Cervidae WHERE id = $1", id)
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

pub async fn get_crime(context: &Context<'_>, id: Uuid) -> Result<Option<Crime>> {
    let crime = query_as!(Crime, "SELECT * FROM Crime WHERE id = $1", id)
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(crime)
}
