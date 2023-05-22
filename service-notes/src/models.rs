use diesel::prelude::*;

use crate::schema::notes;

#[derive(Queryable)]
pub struct DieselNote {
    pub id: uuid::Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub user_id: &'a uuid::Uuid,
    pub title: &'a str,
    pub content: &'a str,
}
