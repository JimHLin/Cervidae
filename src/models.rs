use async_graphql::*;
use chrono::{Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UuidScalar,
    pub name: String,
    pub email: String,
    #[graphql(secret)]
    pub password: String,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Deer {
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Review {
    pub user_id: UuidScalar,
    pub cervidae_id: UuidScalar,
    pub danger_level: i32,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Comment {
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

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Crime {
    pub id: UuidScalar,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTimeScalar>,
    pub updated_at: Option<NaiveDateTimeScalar>,
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

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub struct CrimeCervidae {
    pub crime_id: UuidScalar,
    pub cervidae_id: UuidScalar,
}
