use diesel::prelude::*;

use crate::schema::notes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = notes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DieselNote {
    pub id: Vec<u8>,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub user_id: Vec<u8>,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct UpsertNote<'a> {
    pub id: &'a Vec<u8>,
    pub user_id: &'a Vec<u8>,
    pub title: &'a str,
    pub content: &'a str,
}
