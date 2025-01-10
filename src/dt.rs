use chrono::{DateTime, Local, NaiveDate, TimeZone, Timelike, Utc};
use dioxus::prelude::{server_fn::error::NoCustomError, ServerFnError};
use tracing::error;

const HOUR: u32 = 7;

pub fn get_utc_times_for_date(
    date: NaiveDate,
) -> Result<(DateTime<Utc>, DateTime<Utc>), ServerFnError> {
    let today = date;
    let tomorrow = today.succ_opt().ok_or_else(|| {
        error!("Failed to get tomorrow's date for date: {:?}", today);
        ServerFnError::<NoCustomError>::ServerError("Failed to get tomorrow's date".to_string())
    })?;

    let start = today.and_hms_opt(HOUR, 0, 0).map_or_else(
        || {
            error!("Failed to create start time for date: {:?}", today);
            Err(ServerFnError::<NoCustomError>::ServerError(
                "Failed to create start time".to_string(),
            ))
        },
        |x| Ok(Local.from_local_datetime(&x)),
    )?;

    let end = tomorrow.and_hms_opt(HOUR, 0, 0).map_or_else(
        || {
            error!("Failed to create end time for date: {:?}", tomorrow);
            Err(ServerFnError::<NoCustomError>::ServerError(
                "Failed to create end time".to_string(),
            ))
        },
        |x| Ok(Local.from_local_datetime(&x)),
    )?;

    let start = start.single().unwrap_or_else(|| {
        error!("Failed to convert start time to UTC for date: {:?}", today);
        panic!("Failed to convert start time to UTC");
    });

    let end = end.single().unwrap_or_else(|| {
        error!("Failed to convert end time to UTC for date: {:?}", tomorrow);
        panic!("Failed to convert end time to UTC");
    });

    let start = start.with_timezone(&Utc);
    let end = end.with_timezone(&Utc);

    Ok((start, end))
}

pub fn get_date_for_dt(entry_date: DateTime<Utc>) -> Result<NaiveDate, ServerFnError> {
    let local_date_time = entry_date.with_timezone(&Local);
    let local_date = local_date_time.date_naive();

    if local_date_time.hour() < HOUR {
        Ok(local_date.pred_opt().unwrap_or(local_date))
    } else {
        Ok(local_date)
    }
}
