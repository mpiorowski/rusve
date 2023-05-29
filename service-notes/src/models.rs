use diesel::prelude::*;

use crate::schema::notes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
pub struct UpsertNote<'a> {
    pub id: Option<uuid::Uuid>,
    pub user_id: &'a uuid::Uuid,
    pub title: &'a str,
    pub content: &'a str,
}
