use dioxus_fullstack::ServerFnError;
use jiff::civil::{Date, Time, time};
use jiff::{Timestamp, tz::TimeZone};
use tracing::error;

const DAY_START_TIME: Time = time(6, 30, 0, 0);

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR: i64 = 60 * 60;
const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
const DAYS_PER_WEEK: i64 = 7;

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

/// Display how long ago `entry_time` was, relative to `now`, in compact units
/// ("3h ago"). Anything older than eight weeks falls back to a calendar date,
/// where compact units stop being meaningful.
pub fn display_age(entry_time: Timestamp, now: Timestamp) -> String {
    let seconds = now.duration_since(entry_time).as_secs();
    if seconds < SECONDS_PER_MINUTE {
        "just now".to_string()
    } else if seconds < SECONDS_PER_HOUR {
        format!("{}m ago", seconds / SECONDS_PER_MINUTE)
    } else if seconds < SECONDS_PER_DAY {
        format!("{}h ago", seconds / SECONDS_PER_HOUR)
    } else {
        let days = seconds / SECONDS_PER_DAY;
        if days < 2 * DAYS_PER_WEEK {
            format!("{days}d ago")
        } else if days < 8 * DAYS_PER_WEEK {
            format!("{}w ago", days / DAYS_PER_WEEK)
        } else {
            entry_time
                .to_zoned_datetime(TimeZone::system())
                .strftime("%Y-%m-%d")
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: Timestamp = Timestamp::constant(1_700_000_000, 0);

    fn at_offset(seconds_ago: i64) -> Timestamp {
        Timestamp::constant(NOW.as_second() - seconds_ago, 0)
    }

    #[test]
    fn test_display_age() {
        assert_eq!(display_age(NOW, NOW), "just now");
        assert_eq!(display_age(at_offset(30), NOW), "just now");
        assert_eq!(display_age(at_offset(-30), NOW), "just now");
        assert_eq!(display_age(at_offset(5 * 60), NOW), "5m ago");
        assert_eq!(display_age(at_offset(3 * 60 * 60), NOW), "3h ago");
        assert_eq!(display_age(at_offset(2 * 24 * 60 * 60), NOW), "2d ago");
        assert_eq!(display_age(at_offset(3 * 7 * 24 * 60 * 60), NOW), "3w ago");
    }

    #[test]
    fn test_display_age_old_falls_back_to_date() {
        let old = display_age(at_offset(90 * 24 * 60 * 60), NOW);
        assert!(!old.contains("ago"), "expected a calendar date, got: {old}");
    }
}
