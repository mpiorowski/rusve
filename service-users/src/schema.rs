// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Uuid,
        created -> Timestamptz,
        updated -> Timestamptz,
        deleted -> Nullable<Timestamptz>,
        target_id -> Uuid,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created -> Timestamptz,
        updated -> Timestamptz,
        deleted -> Nullable<Timestamptz>,
        email -> Text,
        role -> Text,
        sub -> Text,
        name -> Text,
        avatar_id -> Nullable<Uuid>,
        payment_id -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
