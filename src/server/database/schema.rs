// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "consumable_unit"))]
    pub struct ConsumableUnit;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "consumption_type"))]
    pub struct ConsumptionType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConsumableUnit;

    consumables (id) {
        id -> Int8,
        name -> Text,
        brand -> Nullable<Text>,
        barcode -> Nullable<Text>,
        is_organic -> Bool,
        unit -> ConsumableUnit,
        comments -> Nullable<Text>,
        created -> Nullable<Timestamptz>,
        destroyed -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    consumption_consumables (parent_id, consumable_id) {
        parent_id -> Int8,
        consumable_id -> Int8,
        quantity -> Nullable<Float8>,
        liquid_mls -> Nullable<Float8>,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConsumptionType;

    consumptions (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        duration -> Interval,
        liquid_mls -> Nullable<Float8>,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        utc_offset -> Int4,
        consumption_type -> ConsumptionType,
    }
}

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    nested_consumables (parent_id, consumable_id) {
        parent_id -> Int8,
        consumable_id -> Int8,
        quantity -> Nullable<Float8>,
        liquid_mls -> Nullable<Float8>,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    poos (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        duration -> Interval,
        urgency -> Int4,
        quantity -> Int4,
        bristol -> Int4,
        colour_hue -> Float4,
        colour_saturation -> Float4,
        colour_value -> Float4,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        utc_offset -> Int4,
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
        password -> Text,
        full_name -> Text,
        oidc_id -> Nullable<Text>,
        email -> Text,
        is_admin -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    wees (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        duration -> Interval,
        urgency -> Int4,
        mls -> Int4,
        colour_hue -> Float4,
        colour_saturation -> Float4,
        colour_value -> Float4,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        utc_offset -> Int4,
    }
}

diesel::joinable!(consumption_consumables -> consumables (consumable_id));
diesel::joinable!(consumption_consumables -> consumptions (parent_id));
diesel::joinable!(consumptions -> users (user_id));
diesel::joinable!(poos -> users (user_id));
diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));
diesel::joinable!(wees -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    consumables,
    consumption_consumables,
    consumptions,
    groups,
    nested_consumables,
    poos,
    session,
    user_groups,
    users,
    wees,
);
