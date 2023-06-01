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
