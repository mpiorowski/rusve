// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Uuid,
        created -> Timestamptz,
        updated -> Timestamptz,
        deleted -> Nullable<Timestamptz>,
        user_id -> Uuid,
        title -> Text,
        content -> Text,
    }
}
