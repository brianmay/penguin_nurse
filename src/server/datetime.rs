//! Helpers for converting between the jiff types used by the frontend models
//! and the shapes the database stores.
//!
//! Timestamps, dates and times go through the `jiff-diesel` wrapper types.
//! Two conversions need more care:
//!
//! * Rows store a UTC instant plus the `utc_offset` it was entered with, which
//!   together reconstruct the `ZonedDateTime` used by the frontend models.
//! * `jiff-diesel`'s `Span` wrapper can only be *read* from a PostgreSQL
//!   interval (encoding an arbitrary span requires a relative datetime), so
//!   durations are written through `PgInterval` directly.

use diesel::pg::data_types::PgInterval;
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp, ZonedDateTime};
use tracing::error;

/// Reconstruct the zoned datetime for a database row from the UTC instant and
/// the `utc_offset` in seconds stored alongside it.
///
/// The offset comes from user input at insert time, so an invalid offset
/// falls back to UTC (with an error logged) rather than failing the whole
/// read.
pub fn zoned_from_parts(timestamp: Timestamp, utc_offset: i32) -> ZonedDateTime {
    let offset = Offset::from_seconds(utc_offset).unwrap_or_else(|err| {
        error!("Invalid utc_offset {utc_offset} stored in database: {err:?}");
        Offset::UTC
    });

    ZonedDateTime::new(timestamp, TimeZone::fixed(offset))
}

/// Convert a PostgreSQL interval, decoded as a span, into a signed duration.
///
/// Our durations are always written as plain time units, so the conversion is
/// exact; a span with calendar units (which cannot happen for our data) falls
/// back to zero with an error logged.
pub fn span_to_signed_duration(span: jiff::Span) -> SignedDuration {
    SignedDuration::try_from(span).unwrap_or_else(|err| {
        error!("Cannot convert database interval with calendar units to a duration: {err:?}");
        SignedDuration::ZERO
    })
}

/// Encode a signed duration as a PostgreSQL interval.
pub fn signed_duration_to_interval(duration: SignedDuration) -> PgInterval {
    let microseconds = i64::try_from(duration.as_micros()).unwrap_or_else(|_| {
        error!("Duration out of range for a PostgreSQL interval: {duration:?}");
        i64::MAX
    });

    PgInterval::from_microseconds(microseconds)
}
