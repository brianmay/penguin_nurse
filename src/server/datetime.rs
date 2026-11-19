//! Conversions between the chrono types used by the diesel database layer and
//! the jiff types used by the frontend models.
//!
//! These exist because diesel's datetime integration is chrono-only for now.
//! Once the diesel layer is migrated to `jiff-diesel`, this module can be
//! removed.
//!
//! The conversions are infallible and fall back to a safe default value (with
//! an error logged) when a value is out of range, mirroring how the database
//! models already handled invalid offsets.

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, TimeDelta, Timelike, Utc};
use jiff::civil::{Date, Time};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use tracing::error;

pub fn chrono_to_timestamp(time: DateTime<Utc>) -> Timestamp {
    Timestamp::new(time.timestamp(), time.timestamp_subsec_nanos() as i32).unwrap_or_else(|err| {
        error!("Failed to convert chrono datetime to jiff timestamp: {time:?}: {err:?}");
        Timestamp::UNIX_EPOCH
    })
}

pub fn timestamp_to_chrono(timestamp: Timestamp) -> DateTime<Utc> {
    let nanosecond = timestamp.as_nanosecond();
    let second = nanosecond.div_euclid(1_000_000_000);
    let subsec_nanosecond = nanosecond.rem_euclid(1_000_000_000);

    DateTime::from_timestamp(second as i64, subsec_nanosecond as u32).unwrap_or_else(|| {
        error!("Failed to convert jiff timestamp to chrono datetime: {timestamp:?}");
        Utc::now()
    })
}

/// Convert a UTC datetime plus a `utc_offset` in seconds (as stored in the
/// database) into a zoned datetime with a fixed-offset time zone.
pub fn chrono_to_zoned(time: DateTime<Utc>, utc_offset: i32) -> ZonedDateTime {
    let offset = Offset::from_seconds(utc_offset).unwrap_or_else(|err| {
        error!("Invalid utc_offset {utc_offset} stored in database: {err:?}");
        Offset::UTC
    });

    ZonedDateTime::new(chrono_to_timestamp(time), TimeZone::fixed(offset))
}

pub fn zoned_to_chrono_utc(zoned: &ZonedDateTime) -> DateTime<Utc> {
    timestamp_to_chrono(zoned.timestamp())
}

/// Convert into a `chrono::DateTime<chrono::FixedOffset>` (as expected by the
/// kiosk API's database functions).
pub fn zoned_to_chrono_fixed(zoned: &ZonedDateTime) -> DateTime<chrono::FixedOffset> {
    let offset = chrono::FixedOffset::east_opt(zoned.offset().seconds()).unwrap_or_else(|| {
        error!(
            "UTC offset out of range for chrono: {}",
            zoned.offset().seconds()
        );
        chrono::FixedOffset::east_opt(0).expect("valid fixed UTC offset")
    });

    zoned_to_chrono_utc(zoned).with_timezone(&offset)
}

pub fn chrono_to_signed_duration(duration: TimeDelta) -> SignedDuration {
    SignedDuration::new(duration.num_seconds(), duration.subsec_nanos())
}

pub fn signed_duration_to_chrono(duration: SignedDuration) -> TimeDelta {
    // chrono requires the fractional nanoseconds to be non-negative, with the
    // sign carried by the whole seconds.
    let (secs, nanos) = {
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();
        if nanos < 0 {
            (secs - 1, (1_000_000_000 + i64::from(nanos)) as u32)
        } else {
            (secs, nanos as u32)
        }
    };

    TimeDelta::new(secs, nanos).unwrap_or_else(|| {
        error!("Failed to convert jiff duration to chrono duration: {duration:?}");
        TimeDelta::zero()
    })
}

pub fn chrono_to_date(date: NaiveDate) -> Date {
    Date::new(
        i16::try_from(date.year()).unwrap_or(0),
        date.month() as i8,
        date.day() as i8,
    )
    .unwrap_or_else(|err| {
        error!("Failed to convert chrono date to jiff date: {date:?}: {err:?}");
        Date::default()
    })
}

pub fn date_to_chrono(date: Date) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year() as i32, date.month() as u32, date.day() as u32)
        .unwrap_or_else(|| {
            error!("Failed to convert jiff date to chrono date: {date:?}");
            NaiveDate::default()
        })
}

pub fn chrono_to_time(time: NaiveTime) -> Time {
    Time::new(
        time.hour() as i8,
        time.minute() as i8,
        time.second() as i8,
        time.nanosecond() as i32,
    )
    .unwrap_or_else(|err| {
        error!("Failed to convert chrono time to jiff time: {time:?}: {err:?}");
        jiff::civil::time(0, 0, 0, 0)
    })
}

pub fn time_to_chrono(time: Time) -> NaiveTime {
    NaiveTime::from_hms_nano_opt(
        time.hour() as u32,
        time.minute() as u32,
        time.second() as u32,
        time.subsec_nanosecond() as u32,
    )
    .unwrap_or_else(|| {
        error!("Failed to convert jiff time to chrono time: {time:?}");
        NaiveTime::default()
    })
}
