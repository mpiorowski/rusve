// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Bytea,
        created -> Timestamptz,
        updated -> Timestamptz,
        deleted -> Nullable<Timestamptz>,
        target_id -> Bytea,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Bytea,
        created -> Timestamptz,
        updated -> Timestamptz,
        deleted -> Nullable<Timestamptz>,
        email -> Text,
        role -> Text,
        sub -> Text,
        name -> Text,
        avatar_id -> Nullable<Bytea>,
        payment_id -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
