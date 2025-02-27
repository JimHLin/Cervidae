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
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct UserOutput {
    pub id: UuidScalar,
    pub name: String,
    pub email: String,
    #[graphql(secret)]
    pub password: String,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
}

impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        UserOutput {
            id: user.id.into(),
            name: user.name,
            email: user.email,
            password: user.password,
            created_at: user.created_at.map(NaiveDateTimeScalar::from),
            updated_at: user.updated_at.map(NaiveDateTimeScalar::from),
        }
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub id: UuidScalar,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UpdateUserInput {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.email.is_none() && self.password.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct DeerOutput {
    pub id: UuidScalar,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub kill_count: Option<i64>,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
    pub created_by: UuidScalar,
    pub updated_by: UuidScalar,
}

impl From<Deer> for DeerOutput {
    fn from(deer: Deer) -> Self {
        DeerOutput {
            id: deer.id.into(),
            name: deer.name,
            description: deer.description,
            image_url: deer.image_url,
            kill_count: deer.kill_count,
            created_at: deer.created_at.map(NaiveDateTimeScalar::from),
            updated_at: deer.updated_at.map(NaiveDateTimeScalar::from),
            created_by: deer.created_by.into(),
            updated_by: deer.updated_by.into(),
        }
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub user_id: Uuid,
    pub cervidae_id: Uuid,
    pub danger_level: i32,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct ReviewOutput {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub danger_level: i32,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
}

impl From<Review> for ReviewOutput {
    fn from(review: Review) -> Self {
        ReviewOutput {
            user_id: review.user_id.into(),
            cervidae_id: review.cervidae_id.into(),
            danger_level: review.danger_level,
            title: review.title,
            body: review.body,
            created_at: review.created_at.map(NaiveDateTimeScalar::from),
            updated_at: review.updated_at.map(NaiveDateTimeScalar::from),
        }
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cervidae_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct CommentOutput {
    pub id: UuidScalar,
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub parent_id: Option<UuidScalar>,
    pub content: String,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CreateCommentInput {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub parent_id: Option<UuidScalar>,
    pub content: String,
}

impl From<Comment> for CommentOutput {
    fn from(comment: Comment) -> Self {
        CommentOutput {
            id: comment.id.into(),
            user_id: comment.user_id.into(),
            cervidae_id: comment.cervidae_id.into(),
            parent_id: comment.parent_id.map(UuidScalar::from),
            content: comment.content,
            created_at: comment.created_at.map(NaiveDateTimeScalar::from),
            updated_at: comment.updated_at.map(NaiveDateTimeScalar::from),
        }
    }
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Crime {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct CrimeOutput {
    pub id: UuidScalar,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
}

impl From<Crime> for CrimeOutput {
    fn from(crime: Crime) -> Self {
        CrimeOutput {
            id: crime.id.into(),
            name: crime.name,
            description: crime.description,
            created_at: crime.created_at.map(NaiveDateTimeScalar::from),
            updated_at: crime.updated_at.map(NaiveDateTimeScalar::from),
        }
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrimeCervidae {
    pub crime_id: Uuid,
    pub cervidae_id: Uuid,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CrimeCervidaeInput {
    pub crime_id: UuidScalar,
    pub cervidae_id: UuidScalar,
}

impl From<CrimeCervidaeInput> for CrimeCervidae {
    fn from(crime_cervidae: CrimeCervidaeInput) -> Self {
        CrimeCervidae {
            crime_id: crime_cervidae.crime_id.into(),
            cervidae_id: crime_cervidae.cervidae_id.into(),
        }
    }
}
