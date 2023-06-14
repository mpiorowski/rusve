use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUser {
    pub id: Vec<u8>,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub email: String,
    pub role: String,
    pub sub: String,
    pub name: String,
    pub avatar_id: Option<uuid::Uuid>,
    pub payment_id: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
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
#[diesel(table_name = crate::schema::files)]
pub struct InsertFile<'a> {
    pub target_id: &'a uuid::Uuid,
    pub name: &'a str,
    pub type_: &'a str,
}
