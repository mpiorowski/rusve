use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DieselUser {
    pub id: uuid::Uuid,
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
