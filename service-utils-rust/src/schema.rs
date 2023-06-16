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
