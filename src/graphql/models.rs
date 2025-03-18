use crate::graphql::storage::*;
use async_graphql::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//Scalar type for foreign types from external libraries
#[derive(Debug, Serialize, Deserialize)]
pub struct UuidScalar(Uuid);

impl From<Uuid> for UuidScalar {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UuidScalar> for Uuid {
    fn from(scalar: UuidScalar) -> Self {
        scalar.0
    }
}

// Add scalar implementations for Uuid and NaiveDateTime
#[Scalar]
impl ScalarType for UuidScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(UuidScalar::from(Uuid::parse_str(value)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaiveDateTimeScalar(NaiveDateTime);

impl NaiveDateTimeScalar {
    pub fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

#[Scalar]
impl ScalarType for NaiveDateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(NaiveDateTimeScalar::from(NaiveDateTime::parse_from_str(
                value,
                "%Y-%m-%d %H:%M:%S",
            )?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
}

#[Object]
impl User {
    pub async fn id(&self) -> UuidScalar {
        UuidScalar::from(self.id)
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn email(&self) -> &str {
        &self.email
    }

    pub async fn password(&self) -> &str {
        &self.password
    }

    pub async fn created_at(&self) -> Option<NaiveDateTimeScalar> {
        self.created_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn updated_at(&self) -> Option<NaiveDateTimeScalar> {
        self.updated_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn reviews(&self, context: &Context<'_>) -> Result<Vec<Review>> {
        let reviews = get_reviews_by_user(context, self.id).await?;
        Ok(reviews)
    }

    pub async fn comments(&self, context: &Context<'_>) -> Result<Vec<Comment>> {
        let comments = get_comments_by_user(context, self.id).await?;
        Ok(comments)
    }
    pub async fn is_admin(&self) -> bool {
        self.is_admin
    }
    pub async fn last_login(&self) -> Option<NaiveDateTimeScalar> {
        self.last_login.map(NaiveDateTimeScalar::from)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub id: UuidScalar,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl UpdateUserInput {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.email.is_none()
    }
}

#[derive(InputObject, Debug, Deserialize)]
pub struct ResetPasswordInput {
    pub id: UuidScalar,
    #[graphql(secret)]
    pub current_password: String,
    #[graphql(secret)]
    pub new_password: String,
}

#[derive(InputObject, Deserialize)]
pub struct LoginInput {
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct Deer {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub kill_count: Option<i64>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[Object]
impl Deer {
    pub async fn id(&self) -> UuidScalar {
        UuidScalar::from(self.id)
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub async fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    pub async fn kill_count(&self) -> Option<i64> {
        self.kill_count
    }

    pub async fn created_at(&self) -> Option<NaiveDateTimeScalar> {
        self.created_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn updated_at(&self) -> Option<NaiveDateTimeScalar> {
        self.updated_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn created_by(&self, context: &Context<'_>) -> Result<User> {
        let created_by = Uuid::from(self.created_by);
        let user = get_user(context, created_by).await?;
        if let Some(user) = user {
            Ok(user)
        } else {
            Err(Error::new("User not found"))
        }
    }

    pub async fn updated_by(&self, context: &Context<'_>) -> Result<User> {
        let updated_by = Uuid::from(self.updated_by);
        let user = get_user(context, updated_by).await?;
        if let Some(user) = user {
            Ok(user)
        } else {
            Err(Error::new("User not found"))
        }
    }

    pub async fn reviews(&self, context: &Context<'_>) -> Result<Vec<Review>> {
        let reviews = get_reviews_by_deer(context, self.id).await?;
        Ok(reviews)
    }

    pub async fn comments(&self, context: &Context<'_>) -> Result<Vec<Comment>> {
        let comments = get_comments_by_deer(context, self.id).await?;
        Ok(comments)
    }

    pub async fn crimes(&self, context: &Context<'_>) -> Result<Vec<Crime>> {
        let crimes = get_crimes_by_deer(context, self.id).await?;
        Ok(crimes)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateDeerInput {
    pub user_id: UuidScalar,
    pub name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub kill_count: Option<i64>,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateDeerInput {
    pub user_id: UuidScalar,
    pub id: UuidScalar,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub kill_count: Option<i64>,
}

impl UpdateDeerInput {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.image_url.is_none()
            && self.kill_count.is_none()
    }
}

#[derive(Deserialize, FromRow)]
pub struct Review {
    pub user_id: Uuid,
    pub cervidae_id: Uuid,
    pub danger_level: i32,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[Object]
impl Review {
    pub async fn user(&self, context: &Context<'_>) -> Result<User> {
        let user = get_user(context, self.user_id).await?;
        if let Some(user) = user {
            Ok(user)
        } else {
            Err(Error::new("User not found"))
        }
    }

    pub async fn deer(&self, context: &Context<'_>) -> Result<Deer> {
        let deer = get_deer(context, self.cervidae_id).await?;
        if let Some(deer) = deer {
            Ok(deer.into())
        } else {
            Err(Error::new("Deer not found"))
        }
    }

    pub async fn danger_level(&self) -> i32 {
        self.danger_level
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn body(&self) -> &str {
        &self.body
    }

    pub async fn created_at(&self) -> Option<NaiveDateTimeScalar> {
        self.created_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn updated_at(&self) -> Option<NaiveDateTimeScalar> {
        self.updated_at.map(NaiveDateTimeScalar::from)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateReviewInput {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub danger_level: i32,
    pub title: String,
    pub body: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateReviewInput {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub danger_level: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl UpdateReviewInput {
    pub fn is_empty(&self) -> bool {
        self.danger_level.is_none() && self.title.is_none() && self.body.is_none()
    }
}

#[derive(Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cervidae_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[Object]
impl Comment {
    pub async fn id(&self) -> UuidScalar {
        UuidScalar::from(self.id)
    }

    pub async fn user(&self, context: &Context<'_>) -> Result<User> {
        let user = get_user(context, self.user_id).await?;
        if let Some(user) = user {
            Ok(user)
        } else {
            Err(Error::new("User not found"))
        }
    }

    pub async fn deer(&self, context: &Context<'_>) -> Result<Deer> {
        let deer = get_deer(context, self.cervidae_id).await?;
        if let Some(deer) = deer {
            Ok(deer)
        } else {
            Err(Error::new("Deer not found"))
        }
    }

    pub async fn parent(&self, context: &Context<'_>) -> Result<Option<Comment>> {
        if let Some(parent_id) = self.parent_id {
            let parent = get_comment(context, parent_id).await?;
            if let Some(parent) = parent {
                Ok(Some(parent))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn content(&self) -> &str {
        &self.content
    }

    pub async fn created_at(&self) -> Option<NaiveDateTimeScalar> {
        self.created_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn updated_at(&self) -> Option<NaiveDateTimeScalar> {
        self.updated_at.map(NaiveDateTimeScalar::from)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateCommentInput {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub parent_id: Option<UuidScalar>,
    pub content: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateCommentInput {
    pub id: UuidScalar,
    pub content: Option<String>,
}

impl UpdateCommentInput {
    pub fn is_empty(&self) -> bool {
        self.content.is_none()
    }
}

#[derive(Deserialize, FromRow)]
pub struct Crime {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[Object]
impl Crime {
    pub async fn id(&self) -> UuidScalar {
        UuidScalar::from(self.id)
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub async fn created_at(&self) -> Option<NaiveDateTimeScalar> {
        self.created_at.map(NaiveDateTimeScalar::from)
    }

    pub async fn updated_at(&self) -> Option<NaiveDateTimeScalar> {
        self.updated_at.map(NaiveDateTimeScalar::from)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateCrimeInput {
    pub name: String,
    pub description: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateCrimeInput {
    pub id: UuidScalar,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateCrimeInput {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none()
    }
}

#[derive(Deserialize, FromRow)]
pub struct CrimeCervidae {
    pub crime_id: Uuid,
    pub cervidae_id: Uuid,
}

#[Object]
impl CrimeCervidae {
    pub async fn crime(&self, context: &Context<'_>) -> Result<Crime> {
        let crime = get_crime(context, self.crime_id).await?;
        if let Some(crime) = crime {
            Ok(crime)
        } else {
            Err(Error::new("Crime not found"))
        }
    }

    pub async fn deer(&self, context: &Context<'_>) -> Result<Deer> {
        let deer = get_deer(context, self.cervidae_id).await?;
        if let Some(deer) = deer {
            Ok(deer)
        } else {
            Err(Error::new("Deer not found"))
        }
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CrimeCervidaeInput {
    pub crime_id: UuidScalar,
    pub cervidae_id: UuidScalar,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Claims {
    pub sub: String,
    pub is_admin: bool,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}
