use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::Utc;
use chrono::{DateTime, TimeDelta};

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone)]
#[db_enum(existing_type_path = "schema::sql_types::ExerciseType")]
pub enum ExerciseType {
    Walking,
    Running,
    Cycling,
    IndoorCycling,
    Jumping,
    Skipping,
    Flying,
    Other,
}

impl From<ExerciseType> for models::ExerciseType {
    fn from(unit: ExerciseType) -> models::ExerciseType {
        match unit {
            ExerciseType::Walking => models::ExerciseType::Walking,
            ExerciseType::Running => models::ExerciseType::Running,
            ExerciseType::Cycling => models::ExerciseType::Cycling,
            ExerciseType::IndoorCycling => models::ExerciseType::IndoorCycling,
            ExerciseType::Jumping => models::ExerciseType::Jumping,
            ExerciseType::Skipping => models::ExerciseType::Skipping,
            ExerciseType::Flying => models::ExerciseType::Flying,
            ExerciseType::Other => models::ExerciseType::Other,
        }
    }
}

impl From<models::ExerciseType> for ExerciseType {
    fn from(unit: models::ExerciseType) -> ExerciseType {
        match unit {
            models::ExerciseType::Walking => ExerciseType::Walking,
            models::ExerciseType::Running => ExerciseType::Running,
            models::ExerciseType::Cycling => ExerciseType::Cycling,
            models::ExerciseType::IndoorCycling => ExerciseType::IndoorCycling,
            models::ExerciseType::Jumping => ExerciseType::Jumping,
            models::ExerciseType::Skipping => ExerciseType::Skipping,
            models::ExerciseType::Flying => ExerciseType::Flying,
            models::ExerciseType::Other => ExerciseType::Other,
        }
    }
}

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::exercises)]
pub struct Exercise {
    pub id: i64,
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub duration: TimeDelta,
    pub location: Option<String>,
    pub distance: Option<bigdecimal::BigDecimal>,
    pub calories: Option<i32>,
    pub rpe: Option<i32>,
    pub exercise_type: ExerciseType,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<Exercise> for crate::models::Exercise {
    fn from(exercise: Exercise) -> Self {
        let timezone =
            chrono::FixedOffset::east_opt(exercise.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = exercise.time.with_timezone(&timezone);

        Self {
            id: models::ExerciseId::new(exercise.id),
            user_id: models::UserId::new(exercise.user_id),
            time,
            duration: exercise.duration,
            location: exercise.location,
            distance: exercise.distance,
            calories: exercise.calories,
            rpe: exercise
                .rpe
                .map(|rpe| rpe.try_into().unwrap_or(models::ExerciseRpe::Rpe1)),
            comments: exercise.comments,
            created_at: exercise.created_at,
            updated_at: exercise.updated_at,
            exercise_type: exercise.exercise_type.into(),
        }
    }
}

pub async fn get_exercises_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Exercise>, diesel::result::Error> {
    // use crate::server::database::schema::exercises::duration as q_duration;
    use crate::server::database::schema::exercises::table;
    use crate::server::database::schema::exercises::time as q_time;
    use crate::server::database::schema::exercises::user_id as q_user_id;

    table
        .select(Exercise::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_exercise_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Exercise>, diesel::result::Error> {
    use crate::server::database::schema::exercises::id as q_id;
    use crate::server::database::schema::exercises::table;
    use crate::server::database::schema::exercises::user_id as q_user_id;

    table
        .select(Exercise::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::exercises)]
pub struct NewExercise<'a> {
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub duration: TimeDelta,
    pub location: Option<&'a str>,
    pub distance: Option<&'a bigdecimal::BigDecimal>,
    pub calories: Option<i32>,
    pub rpe: Option<i32>,
    pub exercise_type: ExerciseType,
    pub comments: Option<&'a str>,
}

impl<'a> NewExercise<'a> {
    pub fn from_front_end(exercise: &'a crate::models::NewExercise) -> Self {
        Self {
            user_id: exercise.user_id.as_inner(),
            time: exercise.time.with_timezone(&Utc),
            utc_offset: exercise.time.offset().local_minus_utc(),
            duration: exercise.duration,
            location: exercise.location.as_deref(),
            distance: exercise.distance.as_ref(),
            calories: exercise.calories,
            rpe: exercise.rpe.map(|rpe| rpe.into()),
            exercise_type: exercise.exercise_type.into(),
            comments: exercise.comments.as_deref(),
        }
    }
}

pub async fn create_exercise(
    conn: &mut DatabaseConnection,
    update: &NewExercise<'_>,
) -> Result<Exercise, diesel::result::Error> {
    diesel::insert_into(schema::exercises::table)
        .values(update)
        .returning(Exercise::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::exercises)]
pub struct ChangeExercise<'a> {
    pub time: Option<DateTime<Utc>>,
    pub utc_offset: Option<i32>,
    pub duration: Option<TimeDelta>,
    pub location: Option<Option<&'a str>>,
    pub distance: Option<Option<&'a bigdecimal::BigDecimal>>,
    pub calories: Option<Option<i32>>,
    pub rpe: Option<Option<i32>>,
    pub exercise_type: Option<ExerciseType>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> ChangeExercise<'a> {
    pub fn from_front_end(exercise: &'a crate::models::ChangeExercise) -> Self {
        Self {
            time: exercise
                .time
                .map(|time| time.with_timezone(&Utc))
                .into_option(),
            utc_offset: exercise
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            duration: exercise.duration.into_option(),
            location: exercise.location.map_inner_deref().into_option(),
            distance: exercise.distance.as_inner_ref().into_option(),
            calories: exercise.calories.into_option(),
            rpe: exercise.rpe.map_inner_into().into_option(),
            exercise_type: exercise.exercise_type.map_into().into_option(),
            comments: exercise.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_exercise(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeExercise<'_>,
) -> Result<Exercise, diesel::result::Error> {
    diesel::update(schema::exercises::table.filter(schema::exercises::id.eq(id)))
        .set(update)
        .returning(Exercise::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_exercise(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::exercises::id as q_id;
    use schema::exercises::table;
    use schema::exercises::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
