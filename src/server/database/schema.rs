// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    session (id) {
        id -> Text,
        data -> Jsonb,
        expiry_date -> Timestamptz,
    }
}

diesel::table! {
    user_groups (user_id, group_id) {
        user_id -> Int8,
        group_id -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        full_name -> Text,
        password -> Text,
        oidc_id -> Nullable<Text>,
        email -> Text,
        is_admin -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    session,
    user_groups,
    users,
);
