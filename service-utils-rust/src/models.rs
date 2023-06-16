use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct DieselFile {
    pub id: Vec<u8>,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub deleted: Option<time::OffsetDateTime>,
    pub target_id: Vec<u8>,
    pub name: String,
    pub type_: String,

}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct InsertFile<'a> {
    pub id: Vec<u8>,
    pub target_id: Vec<u8>,
    pub name: &'a str,
    pub type_: &'a str,
}
