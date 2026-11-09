use std::collections::HashMap;

use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, TimeZone, Utc};

use crate::models;
use crate::server::database::models::consumables::Consumable;
use crate::server::database::models::consumptions::ConsumptionType;
use crate::server::database::models::schedule_consumables::ScheduleConsumable;
use crate::server::database::{connection::DatabaseConnection, schema};

#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedules)]
pub struct Schedule {
    pub id: i64,
    pub user_id: i64,
    pub time_of_day: NaiveTime,
    pub consumption_type: ConsumptionType,
    pub comments: Option<String>,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Schedule> for crate::models::Schedule {
    fn from(schedule: Schedule) -> Self {
        Self {
            id: models::ScheduleId::new(schedule.id),
            user_id: models::UserId::new(schedule.user_id),
            time_of_day: schedule.time_of_day,
            consumption_type: schedule.consumption_type.into(),
            comments: schedule.comments,
            active: schedule.active,
            created_at: schedule.created_at,
            updated_at: schedule.updated_at,
        }
    }
}

/// The local time-of-day at which a new day starts, as used by the timeline
/// views. Entries before this time belong to the previous day.
const DAY_START_TIME: NaiveTime = NaiveTime::from_hms_opt(6, 30, 0).unwrap();

/// Returns the app-day containing `now`: the local date the day belongs to,
/// the date of the next day, and the UTC window (day 06:30 local to next day
/// 06:30 local).
///
/// The offset of `now` is used for all conversions; a `FixedOffset` has no DST
/// transitions, so the conversions are always unambiguous.
fn day_window(now: DateTime<FixedOffset>) -> (NaiveDate, NaiveDate, DateTime<Utc>, DateTime<Utc>) {
    let offset = *now.offset();
    let now_local = now.naive_local();
    let day_date = if now_local.time() < DAY_START_TIME {
        now_local.date().pred_opt().unwrap_or(now_local.date())
    } else {
        now_local.date()
    };
    let next_day_date = day_date.succ_opt().unwrap_or(day_date);

    let start = offset
        .from_local_datetime(
            &day_date
                .and_hms_opt(6, 30, 0)
                .expect("06:30:00 is a valid time"),
        )
        .unwrap()
        .with_timezone(&Utc);
    let end = offset
        .from_local_datetime(
            &next_day_date
                .and_hms_opt(6, 30, 0)
                .expect("06:30:00 is a valid time"),
        )
        .unwrap()
        .with_timezone(&Utc);

    (day_date, next_day_date, start, end)
}

/// A scheduled occurrence of a consumable: the local date and time it falls
/// on, and the id of the schedule it belongs to. Ordering is by date/time,
/// then schedule id.
type Occurrence = ((NaiveDate, NaiveTime), i64);

/// Returns every active schedule for the user, with the status of its
/// occurrence for the app-day containing `now`.
///
/// A scheduled occurrence is recorded if one of the day's consumptions of
/// that consumable is assigned to it: each consumption satisfies the most
/// recent unsatisfied occurrence scheduled at or before its time, and a
/// consumption before every occurrence satisfies the next one. Recording
/// one dose therefore only cancels that dose, not later scheduled doses of
/// the same consumable, and ad-hoc consumption still counts.
///
/// The schedule is due if its time-of-day has passed within the day window
/// (a schedule before 06:30 belongs to the end of the window, i.e. the next
/// calendar day).
pub async fn get_today_schedule_status(
    conn: &mut DatabaseConnection,
    user_id: i64,
    now: DateTime<FixedOffset>,
) -> Result<Vec<models::ScheduleStatus>, diesel::result::Error> {
    let (day_date, next_day_date, start, end) = day_window(now);

    let schedules: Vec<Schedule> = {
        use crate::server::database::schema::schedules::active as q_active;
        use crate::server::database::schema::schedules::table;
        use crate::server::database::schema::schedules::time_of_day as q_time_of_day;
        use crate::server::database::schema::schedules::user_id as q_user_id;

        table
            .select(Schedule::as_select())
            .filter(q_user_id.eq(user_id))
            .filter(q_active.eq(true))
            .order(q_time_of_day.asc())
            .load(conn)
            .await?
    };

    let items: Vec<(ScheduleConsumable, Consumable)> = ScheduleConsumable::belonging_to(&schedules)
        .inner_join(schema::consumables::table)
        .order(schema::consumables::name.asc())
        .load(conn)
        .await?;

    // Local times of the day's consumptions of each consumable, in order.
    let consumption_times: HashMap<i64, Vec<(NaiveDate, NaiveTime)>> = {
        use crate::server::database::schema::consumption_consumables::consumable_id as q_consumable_id;
        use crate::server::database::schema::consumption_consumables::table as consumption_consumables_table;
        use crate::server::database::schema::consumptions::dsl as q_consumptions;

        let rows: Vec<(i64, DateTime<Utc>)> = consumption_consumables_table
            .inner_join(schema::consumptions::table)
            .filter(q_consumptions::user_id.eq(user_id))
            .filter(q_consumptions::time.ge(start))
            .filter(q_consumptions::time.lt(end))
            .select((q_consumable_id, q_consumptions::time))
            .load(conn)
            .await?;

        let offset = *now.offset();
        let mut times: HashMap<i64, Vec<(NaiveDate, NaiveTime)>> = HashMap::new();
        for (consumable_id, time) in rows {
            let local = time.with_timezone(&offset);
            times
                .entry(consumable_id)
                .or_default()
                .push((local.date_naive(), local.time()));
        }
        for list in times.values_mut() {
            list.sort();
        }
        times
    };

    // The local date and time of each schedule's occurrence within the day
    // window (a schedule before 06:30 belongs to the end of the window, i.e.
    // the next calendar day).
    let occurrence_by_schedule: HashMap<i64, (NaiveDate, NaiveTime)> = schedules
        .iter()
        .map(|schedule| {
            (
                schedule.id,
                (
                    if schedule.time_of_day < DAY_START_TIME {
                        next_day_date
                    } else {
                        day_date
                    },
                    schedule.time_of_day,
                ),
            )
        })
        .collect();

    // Which occurrences are satisfied by the day's consumptions. Each
    // consumption of a consumable satisfies the most recent unsatisfied
    // occurrence scheduled at or before its time; a consumption before every
    // occurrence satisfies the next one.
    let recorded: HashMap<(i64, i64), bool> = {
        let mut occurrences: HashMap<i64, Vec<Occurrence>> = HashMap::new();
        for (nested, _consumable) in &items {
            if let Some(&occurrence) = occurrence_by_schedule.get(&nested.parent_id) {
                occurrences
                    .entry(nested.consumable_id)
                    .or_default()
                    .push((occurrence, nested.parent_id));
            }
        }
        for list in occurrences.values_mut() {
            list.sort();
        }

        let mut recorded = HashMap::new();
        for (consumable_id, occurrences) in &occurrences {
            let mut satisfied = vec![false; occurrences.len()];
            if let Some(times) = consumption_times.get(consumable_id) {
                for &time in times {
                    let index = occurrences
                        .iter()
                        .enumerate()
                        .rposition(|(index, (occurrence, _))| {
                            !satisfied[index] && *occurrence <= time
                        })
                        .or_else(|| (0..occurrences.len()).find(|index| !satisfied[*index]));
                    if let Some(index) = index {
                        satisfied[index] = true;
                    }
                }
            }
            for ((_, schedule_id), satisfied) in occurrences.iter().zip(&satisfied) {
                recorded.insert((*consumable_id, *schedule_id), *satisfied);
            }
        }
        recorded
    };

    let grouped: Vec<Vec<(ScheduleConsumable, Consumable)>> = items.grouped_by(&schedules);

    let now_local = now.naive_local();

    let result: Vec<_> = schedules
        .into_iter()
        .zip(grouped)
        .map(|(schedule, items)| {
            let due = if let Some(&occurrence) = occurrence_by_schedule.get(&schedule.id)
                && (now_local.date(), now_local.time()) >= occurrence
            {
                models::ScheduleDue::Due
            } else {
                models::ScheduleDue::Upcoming
            };

            let items: Vec<models::ScheduleItemStatus> = items
                .into_iter()
                .map(|(nested, consumable)| {
                    let recorded = recorded
                        .get(&(consumable.id, schedule.id))
                        .copied()
                        .unwrap_or(false);
                    models::ScheduleItemStatus {
                        item: models::ScheduleItem::new(nested.into(), consumable.into()),
                        recorded,
                    }
                })
                .collect();

            models::ScheduleStatus {
                schedule: schedule.into(),
                due,
                items,
            }
        })
        .collect();

    Ok(result)
}

/// Returns all schedules for the user, active and inactive, ordered by
/// time-of-day.
pub async fn get_schedules(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Vec<(Schedule, Vec<(ScheduleConsumable, Consumable)>)>, diesel::result::Error> {
    let schedules: Vec<Schedule> = {
        use crate::server::database::schema::schedules::id as q_id;
        use crate::server::database::schema::schedules::table;
        use crate::server::database::schema::schedules::time_of_day as q_time_of_day;
        use crate::server::database::schema::schedules::user_id as q_user_id;

        table
            .select(Schedule::as_select())
            .filter(q_user_id.eq(user_id))
            .order((q_time_of_day.asc(), q_id.asc()))
            .load(conn)
            .await?
    };

    let nested: Vec<(ScheduleConsumable, Consumable)> =
        ScheduleConsumable::belonging_to(&schedules)
            .inner_join(schema::consumables::table)
            .order(schema::consumables::name.asc())
            .load(conn)
            .await?;

    let result: Vec<_> = nested
        .grouped_by(&schedules)
        .into_iter()
        .zip(schedules)
        .map(|(a, b)| (b, a))
        .collect();

    Ok(result)
}

pub async fn get_schedule_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Schedule>, diesel::result::Error> {
    use crate::server::database::schema::schedules::id as q_id;
    use crate::server::database::schema::schedules::table;
    use crate::server::database::schema::schedules::user_id as q_user_id;

    table
        .select(Schedule::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedules)]
pub struct NewSchedule<'a> {
    pub user_id: i64,
    pub time_of_day: NaiveTime,
    pub consumption_type: ConsumptionType,
    pub comments: Option<&'a str>,
    pub active: bool,
}

impl<'a> NewSchedule<'a> {
    pub fn from_front_end(schedule: &'a crate::models::NewSchedule) -> Self {
        Self {
            user_id: schedule.user_id.as_inner(),
            time_of_day: schedule.time_of_day,
            consumption_type: schedule.consumption_type.into(),
            comments: schedule.comments.as_deref(),
            active: schedule.active,
        }
    }
}

pub async fn create_schedule(
    conn: &mut DatabaseConnection,
    update: &NewSchedule<'_>,
) -> Result<Schedule, diesel::result::Error> {
    diesel::insert_into(schema::schedules::table)
        .values(update)
        .returning(Schedule::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedules)]
pub struct ChangeSchedule<'a> {
    pub time_of_day: Option<NaiveTime>,
    pub consumption_type: Option<ConsumptionType>,
    pub comments: Option<Option<&'a str>>,
    pub active: Option<bool>,
}

impl<'a> ChangeSchedule<'a> {
    pub fn from_front_end(schedule: &'a crate::models::ChangeSchedule) -> Self {
        Self {
            time_of_day: schedule.time_of_day.into_option(),
            consumption_type: schedule.consumption_type.map_into().into_option(),
            comments: schedule.comments.map_inner_deref().into_option(),
            active: schedule.active.into_option(),
        }
    }
}

pub async fn update_schedule(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeSchedule<'_>,
) -> Result<Schedule, diesel::result::Error> {
    diesel::update(schema::schedules::table.filter(schema::schedules::id.eq(id)))
        .set(update)
        .returning(Schedule::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_schedule(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::schedules::id as q_id;
    use schema::schedules::table;
    use schema::schedules::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
