use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use dioxus::prelude::{ServerFnError, server_fn::error::NoCustomError};
use tap::Pipe;
use tracing::error;

const DAY_TIME: NaiveTime = NaiveTime::from_hms_opt(6, 30, 0).unwrap();

pub fn get_utc_times_for_date(
    date: NaiveDate,
) -> Result<(DateTime<Utc>, DateTime<Utc>), ServerFnError> {
    let today = date;
    let tomorrow = today.succ_opt().ok_or_else(|| {
        error!("Failed to get tomorrow's date for date: {:?}", today);
        ServerFnError::<NoCustomError>::ServerError("Failed to get tomorrow's date".to_string())
    })?;

    let start = today
        .and_time(DAY_TIME)
        .pipe(|x| Local.from_local_datetime(&x));
    let end = tomorrow
        .and_time(DAY_TIME)
        .pipe(|x| Local.from_local_datetime(&x));

    let start = start.single().ok_or_else(|| {
        error!("Failed to convert start time to UTC for date: {:?}", today);
        ServerFnError::<NoCustomError>::ServerError("Failed to convert start time".to_string())
    })?;

    let end = end.single().ok_or_else(|| {
        error!("Failed to convert end time to UTC for date: {:?}", tomorrow);
        ServerFnError::<NoCustomError>::ServerError("Failed to convert end time".to_string())
    })?;

    let start = start.with_timezone(&Utc);
    let end = end.with_timezone(&Utc);

    Ok((start, end))
}

pub fn get_date_for_dt(entry_date: DateTime<Utc>) -> NaiveDate {
    let local_date_time = entry_date.with_timezone(&Local);
    let local_date = local_date_time.date_naive();

    if local_date_time.time() < DAY_TIME {
        local_date.pred_opt().unwrap_or(local_date)
    } else {
        local_date
    }
}

pub fn display_date(entry_date: NaiveDate) -> String {
    entry_date.format("%A, %-d %B, %C%y").to_string()
}
