use diesel::prelude::*;

use crate::proto::{User, UserRole};

#[derive(Queryable)]
pub struct DieselUser {
    pub id: uuid::Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub email: String,
    pub role: String,
    pub sub: String,
    pub name: String,
    pub avatar: Option<uuid::Uuid>,
    pub payment_id: String,
}
