// @generated automatically by Diesel CLI.

diesel::table! {
    session (id) {
        id -> Text,
        data -> Jsonb,
        expiry_date -> Timestamptz,
    }
}
