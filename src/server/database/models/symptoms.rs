use crate::models::{SymptomId, UserId};
use crate::server::database::{connection::DatabaseConnection, schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::symptoms)]
pub struct Symptom {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub utc_offset: i32,
    pub appetite_loss: i32,
    pub fever: i32,
    pub cough: i32,
    pub sore_throat: i32,
    pub nasal_symptom: i32,
    pub nasal_symptom_description: Option<String>,
    pub sneezing: i32,
    pub heart_burn: i32,
    pub abdominal_pain: i32,
    pub abdominal_pain_location: Option<String>,
    pub diarrhea: i32,
    pub constipation: i32,
    pub lower_back_pain: i32,
    pub upper_back_pain: i32,
    pub neck_pain: i32,
    pub joint_pain: i32,
    pub headache: i32,
    pub nausea: i32,
    pub dizziness: i32,
    pub stomach_ache: i32,
    pub chest_pain: i32,
    pub shortness_of_breath: i32,
    pub fatigue: i32,
    pub anxiety: i32,
    pub depression: i32,
    pub insomnia: i32,
    pub shoulder_pain: i32,
    pub hand_pain: i32,
    pub foot_pain: i32,
    pub wrist_pain: i32,
    pub dental_pain: i32,
    pub eye_pain: i32,
    pub ear_pain: i32,
    pub feeling_hot: i32,
    pub feeling_cold: i32,
    pub feeling_thirsty: i32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<Symptom> for crate::models::Symptom {
    fn from(symptom: Symptom) -> Self {
        let timezone =
            chrono::FixedOffset::east_opt(symptom.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = symptom.time.with_timezone(&timezone);

        Self {
            id: SymptomId::new(symptom.id),
            user_id: UserId::new(symptom.user_id),
            time,
            appetite_loss: symptom.appetite_loss,
            fever: symptom.fever,
            cough: symptom.cough,
            sore_throat: symptom.sore_throat,
            nasal_symptom: symptom.nasal_symptom,
            nasal_symptom_description: symptom.nasal_symptom_description,
            sneezing: symptom.sneezing,
            heart_burn: symptom.heart_burn,
            abdominal_pain: symptom.abdominal_pain,
            abdominal_pain_location: symptom.abdominal_pain_location,
            diarrhea: symptom.diarrhea,
            constipation: symptom.constipation,
            lower_back_pain: symptom.lower_back_pain,
            upper_back_pain: symptom.upper_back_pain,
            neck_pain: symptom.neck_pain,
            joint_pain: symptom.joint_pain,
            headache: symptom.headache,
            nausea: symptom.nausea,
            dizziness: symptom.dizziness,
            stomach_ache: symptom.stomach_ache,
            chest_pain: symptom.chest_pain,
            shortness_of_breath: symptom.shortness_of_breath,
            fatigue: symptom.fatigue,
            anxiety: symptom.anxiety,
            depression: symptom.depression,
            insomnia: symptom.insomnia,
            created_at: symptom.created_at,
            updated_at: symptom.updated_at,
            shoulder_pain: symptom.shoulder_pain,
            hand_pain: symptom.hand_pain,
            foot_pain: symptom.foot_pain,
            wrist_pain: symptom.wrist_pain,
            dental_pain: symptom.dental_pain,
            eye_pain: symptom.eye_pain,
            ear_pain: symptom.ear_pain,
            feeling_hot: symptom.feeling_hot,
            feeling_cold: symptom.feeling_cold,
            feeling_thirsty: symptom.feeling_thirsty,
            comments: symptom.comments,
        }
    }
}

pub async fn get_symptoms_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Symptom>, diesel::result::Error> {
    // use crate::server::database::schema::symptoms::duration as q_duration;
    use crate::server::database::schema::symptoms::table;
    use crate::server::database::schema::symptoms::time as q_time;
    use crate::server::database::schema::symptoms::user_id as q_user_id;

    table
        .select(Symptom::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_symptom_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Symptom>, diesel::result::Error> {
    use crate::server::database::schema::symptoms::id as q_id;
    use crate::server::database::schema::symptoms::table;
    use crate::server::database::schema::symptoms::user_id as q_user_id;

    table
        .select(Symptom::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::symptoms)]
pub struct NewSymptom<'a> {
    user_id: i64,
    time: chrono::DateTime<chrono::Utc>,
    utc_offset: i32,
    appetite_loss: i32,
    fever: i32,
    cough: i32,
    sore_throat: i32,
    nasal_symptom: i32,
    nasal_symptom_description: Option<&'a str>,
    sneezing: i32,
    heart_burn: i32,
    abdominal_pain: i32,
    abdominal_pain_location: Option<&'a str>,
    diarrhea: i32,
    constipation: i32,
    lower_back_pain: i32,
    upper_back_pain: i32,
    neck_pain: i32,
    joint_pain: i32,
    headache: i32,
    nausea: i32,
    dizziness: i32,
    stomach_ache: i32,
    chest_pain: i32,
    shortness_of_breath: i32,
    fatigue: i32,
    anxiety: i32,
    depression: i32,
    insomnia: i32,
    shoulder_pain: i32,
    hand_pain: i32,
    foot_pain: i32,
    wrist_pain: i32,
    dental_pain: i32,
    eye_pain: i32,
    ear_pain: i32,
    feeling_hot: i32,
    feeling_cold: i32,
    feeling_thirsty: i32,
    comments: Option<&'a str>,
}

impl<'a> NewSymptom<'a> {
    pub fn from_front_end(symptom: &'a crate::models::NewSymptom) -> Self {
        Self {
            user_id: symptom.user_id.as_inner(),
            time: symptom.time.with_timezone(&Utc),
            utc_offset: symptom.time.offset().local_minus_utc(),
            appetite_loss: symptom.appetite_loss,
            fever: symptom.fever,
            cough: symptom.cough,
            sore_throat: symptom.sore_throat,
            nasal_symptom: symptom.nasal_symptom,
            nasal_symptom_description: symptom.nasal_symptom_description.as_deref(),
            sneezing: symptom.sneezing,
            heart_burn: symptom.heart_burn,
            abdominal_pain: symptom.abdominal_pain,
            abdominal_pain_location: symptom.abdominal_pain_location.as_deref(),
            diarrhea: symptom.diarrhea,
            constipation: symptom.constipation,
            lower_back_pain: symptom.lower_back_pain,
            upper_back_pain: symptom.upper_back_pain,
            neck_pain: symptom.neck_pain,
            joint_pain: symptom.joint_pain,
            headache: symptom.headache,
            nausea: symptom.nausea,
            dizziness: symptom.dizziness,
            stomach_ache: symptom.stomach_ache,
            chest_pain: symptom.chest_pain,
            shortness_of_breath: symptom.shortness_of_breath,
            fatigue: symptom.fatigue,
            anxiety: symptom.anxiety,
            depression: symptom.depression,
            insomnia: symptom.insomnia,
            shoulder_pain: symptom.shoulder_pain,
            hand_pain: symptom.hand_pain,
            foot_pain: symptom.foot_pain,
            wrist_pain: symptom.wrist_pain,
            dental_pain: symptom.dental_pain,
            eye_pain: symptom.eye_pain,
            ear_pain: symptom.ear_pain,
            feeling_hot: symptom.feeling_hot,
            feeling_cold: symptom.feeling_cold,
            feeling_thirsty: symptom.feeling_thirsty,
            comments: symptom.comments.as_deref(),
        }
    }
}

pub async fn create_symptom(
    conn: &mut DatabaseConnection,
    update: &NewSymptom<'_>,
) -> Result<Symptom, diesel::result::Error> {
    diesel::insert_into(schema::symptoms::table)
        .values(update)
        .returning(Symptom::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::symptoms)]
pub struct ChangeSymptom<'a> {
    time: Option<chrono::DateTime<chrono::Utc>>,
    utc_offset: Option<i32>,
    appetite_loss: Option<i32>,
    fever: Option<i32>,
    cough: Option<i32>,
    sore_throat: Option<i32>,
    nasal_symptom: Option<i32>,
    nasal_symptom_description: Option<Option<&'a str>>,
    sneezing: Option<i32>,
    heart_burn: Option<i32>,
    abdominal_pain: Option<i32>,
    abdominal_pain_location: Option<Option<&'a str>>,
    diarrhea: Option<i32>,
    constipation: Option<i32>,
    lower_back_pain: Option<i32>,
    upper_back_pain: Option<i32>,
    neck_pain: Option<i32>,
    joint_pain: Option<i32>,
    headache: Option<i32>,
    nausea: Option<i32>,
    dizziness: Option<i32>,
    stomach_ache: Option<i32>,
    chest_pain: Option<i32>,
    shortness_of_breath: Option<i32>,
    fatigue: Option<i32>,
    anxiety: Option<i32>,
    depression: Option<i32>,
    insomnia: Option<i32>,
    shoulder_pain: Option<i32>,
    hand_pain: Option<i32>,
    foot_pain: Option<i32>,
    wrist_pain: Option<i32>,
    dental_pain: Option<i32>,
    eye_pain: Option<i32>,
    ear_pain: Option<i32>,
    feeling_hot: Option<i32>,
    feeling_cold: Option<i32>,
    feeling_thirsty: Option<i32>,
    comments: Option<Option<&'a str>>,
}

impl<'a> ChangeSymptom<'a> {
    pub fn from_front_end(symptom: &'a crate::models::ChangeSymptom) -> Self {
        Self {
            time: symptom
                .time
                .map(|time| time.with_timezone(&Utc))
                .into_option(),
            utc_offset: symptom
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            appetite_loss: symptom.appetite_loss.into_option(),
            fever: symptom.fever.into_option(),
            cough: symptom.cough.into_option(),
            sore_throat: symptom.sore_throat.into_option(),
            nasal_symptom: symptom.nasal_symptom.into_option(),
            nasal_symptom_description: symptom
                .nasal_symptom_description
                .map_inner_deref()
                .into_option(),
            sneezing: symptom.sneezing.into_option(),
            heart_burn: symptom.heart_burn.into_option(),
            abdominal_pain: symptom.abdominal_pain.into_option(),
            abdominal_pain_location: symptom
                .abdominal_pain_location
                .map_inner_deref()
                .into_option(),
            diarrhea: symptom.diarrhea.into_option(),
            constipation: symptom.constipation.into_option(),
            lower_back_pain: symptom.lower_back_pain.into_option(),
            upper_back_pain: symptom.upper_back_pain.into_option(),
            neck_pain: symptom.neck_pain.into_option(),
            joint_pain: symptom.joint_pain.into_option(),
            headache: symptom.headache.into_option(),
            nausea: symptom.nausea.into_option(),
            dizziness: symptom.dizziness.into_option(),
            stomach_ache: symptom.stomach_ache.into_option(),
            chest_pain: symptom.chest_pain.into_option(),
            shortness_of_breath: symptom.shortness_of_breath.into_option(),
            fatigue: symptom.fatigue.into_option(),
            anxiety: symptom.anxiety.into_option(),
            depression: symptom.depression.into_option(),
            insomnia: symptom.insomnia.into_option(),
            shoulder_pain: symptom.shoulder_pain.into_option(),
            hand_pain: symptom.hand_pain.into_option(),
            foot_pain: symptom.foot_pain.into_option(),
            wrist_pain: symptom.wrist_pain.into_option(),
            dental_pain: symptom.dental_pain.into_option(),
            eye_pain: symptom.eye_pain.into_option(),
            ear_pain: symptom.ear_pain.into_option(),
            feeling_hot: symptom.feeling_hot.into_option(),
            feeling_cold: symptom.feeling_cold.into_option(),
            feeling_thirsty: symptom.feeling_thirsty.into_option(),
            comments: symptom.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_symptom(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeSymptom<'_>,
) -> Result<Symptom, diesel::result::Error> {
    diesel::update(schema::symptoms::table.filter(schema::symptoms::id.eq(id)))
        .set(update)
        .returning(Symptom::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_symptom(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::symptoms::id as q_id;
    use schema::symptoms::table;
    use schema::symptoms::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
