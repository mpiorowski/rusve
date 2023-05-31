use diesel::prelude::*;

use crate::schema::files;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselFile {
    pub id: uuid::Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub target_id: uuid::Uuid,
    pub name: String,
    pub type_: String,

}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct UpsertFile<'a> {
    pub id: Option<uuid::Uuid>,
    pub target_id: &'a uuid::Uuid,
    pub name: &'a str,
    pub type_: &'a str,
}
