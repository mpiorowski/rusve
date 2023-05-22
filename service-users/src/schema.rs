// @generated automatically by Diesel CLI.

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
        avatar -> Nullable<Uuid>,
        payment_id -> Text,
    }
}
