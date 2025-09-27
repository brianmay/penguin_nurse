// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "consumable_unit"))]
    pub struct ConsumableUnit;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "consumption_type"))]
    pub struct ConsumptionType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "exercise_type"))]
    pub struct ExerciseType;
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
    use diesel::sql_types::*;
    use super::sql_types::ExerciseType;

    exercises (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
        duration -> Interval,
        location -> Nullable<Text>,
        distance -> Nullable<Numeric>,
        calories -> Nullable<Int4>,
        rpe -> Nullable<Int4>,
        exercise_type -> ExerciseType,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    health_metrics (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
        pulse -> Nullable<Int4>,
        blood_glucose -> Nullable<Numeric>,
        systolic_bp -> Nullable<Int4>,
        diastolic_bp -> Nullable<Int4>,
        weight -> Nullable<Numeric>,
        height -> Nullable<Int4>,
        comments -> Nullable<Text>,
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
    notes (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
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
    refluxs (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
        duration -> Interval,
        location -> Nullable<Text>,
        severity -> Int4,
        comments -> Nullable<Text>,
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
    symptoms (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
        appetite_loss -> Int4,
        fever -> Int4,
        cough -> Int4,
        sore_throat -> Int4,
        sneezing -> Int4,
        heart_burn -> Int4,
        abdominal_pain -> Int4,
        abdominal_pain_location -> Nullable<Text>,
        diarrhea -> Int4,
        constipation -> Int4,
        lower_back_pain -> Int4,
        upper_back_pain -> Int4,
        neck_pain -> Int4,
        joint_pain -> Int4,
        headache -> Int4,
        nausea -> Int4,
        dizziness -> Int4,
        stomach_ache -> Int4,
        chest_pain -> Int4,
        shortness_of_breath -> Int4,
        fatigue -> Int4,
        anxiety -> Int4,
        depression -> Int4,
        insomnia -> Int4,
        comments -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        shoulder_pain -> Int4,
        hand_pain -> Int4,
        foot_pain -> Int4,
        wrist_pain -> Int4,
        dental_pain -> Int4,
        eye_pain -> Int4,
        ear_pain -> Int4,
        feeling_hot -> Int4,
        feeling_cold -> Int4,
        nasal_symptom -> Int4,
        nasal_symptom_description -> Nullable<Text>,
        feeling_thirsty -> Int4,
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
    wee_urges (id) {
        id -> Int8,
        user_id -> Int8,
        time -> Timestamptz,
        utc_offset -> Int4,
        urgency -> Int4,
        comments -> Nullable<Text>,
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
        leakage -> Int4,
    }
}

diesel::joinable!(consumption_consumables -> consumables (consumable_id));
diesel::joinable!(consumption_consumables -> consumptions (parent_id));
diesel::joinable!(consumptions -> users (user_id));
diesel::joinable!(exercises -> users (user_id));
diesel::joinable!(health_metrics -> users (user_id));
diesel::joinable!(notes -> users (user_id));
diesel::joinable!(poos -> users (user_id));
diesel::joinable!(refluxs -> users (user_id));
diesel::joinable!(symptoms -> users (user_id));
diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));
diesel::joinable!(wee_urges -> users (user_id));
diesel::joinable!(wees -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    consumables,
    consumption_consumables,
    consumptions,
    exercises,
    groups,
    health_metrics,
    nested_consumables,
    notes,
    poos,
    refluxs,
    session,
    symptoms,
    user_groups,
    users,
    wee_urges,
    wees,
);
