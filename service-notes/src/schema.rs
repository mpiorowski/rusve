// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        #[max_length = 16]
        id -> Binary,
        created -> Timestamp,
        updated -> Timestamp,
        deleted -> Nullable<Timestamp>,
        #[max_length = 16]
        user_id -> Binary,
        title -> Text,
        content -> Text,
    }
}
