use crate::graphql::models::*;
use async_graphql::{Context, Object, Result};
use sqlx::{self, query, query_as, Encode, PgPool, Postgres, QueryBuilder, Type};
use uuid::Uuid;

pub async fn get_user(context: &Context<'_>, id: Uuid) -> Result<Option<UserOutput>> {
    let user = query_as!(User, "SELECT * FROM Users WHERE id = $1", id)
        .fetch_optional(context.data_unchecked::<PgPool>())
        .await
        .map_err(|e| e.to_string())?;

    Ok(user.map(UserOutput::from))
}
