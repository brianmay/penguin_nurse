use dioxus_fullstack::ServerFnError;
use jiff::civil::{Date, Time, time};
use jiff::{Timestamp, tz::TimeZone};
use tracing::error;

const DAY_START_TIME: Time = time(6, 30, 0, 0);

pub fn get_utc_times_for_date(entry_date: Date) -> Result<(Timestamp, Timestamp), ServerFnError> {
    let tomorrow = entry_date.tomorrow().map_err(|err| {
        error!("Failed to get tomorrow's date for date: {entry_date:?}: {err:?}");
        ServerFnError::new("Failed to get tomorrow's date".to_string())
    })?;

    let timezone = TimeZone::system();

    let start = timezone
        .to_ambiguous_zoned_datetime(entry_date.to_datetime(DAY_START_TIME))
        .unambiguous()
        .map_err(|err| {
            error!("Failed to convert start time to UTC for date: {entry_date:?}: {err:?}");
            ServerFnError::new("Failed to convert start time".to_string())
        })?
        .timestamp();

    let end = timezone
        .to_ambiguous_zoned_datetime(tomorrow.to_datetime(DAY_START_TIME))
        .unambiguous()
        .map_err(|err| {
            error!("Failed to convert end time to UTC for date: {tomorrow:?}: {err:?}");
            ServerFnError::new("Failed to convert end time".to_string())
        })?
        .timestamp();

    Ok((start, end))
}

pub fn get_date_for_timestamp(entry_date: Timestamp) -> Date {
    let local = entry_date.to_zoned_datetime(TimeZone::system());
    let local_date = local.date();

    if local.time() < DAY_START_TIME {
        local_date.yesterday().unwrap_or(local_date)
    } else {
        local_date
    }
}

pub fn display_date(entry_date: Date) -> String {
    entry_date.strftime("%A, %-d %B, %C%y").to_string()
}
