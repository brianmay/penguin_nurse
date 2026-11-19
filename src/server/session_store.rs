use async_trait::async_trait;
use tap::Pipe;
use thiserror::Error;
use tower_sessions::cookie::time;
use tower_sessions::session::Record;
use tower_sessions::{ExpiredDeletion, SessionStore};
use tower_sessions::{session::Id, session_store};

use crate::server::database::models::session::{
    create_or_update_session, delete_expired, delete_session, load_session, session_exists,
};

use super::database;
use super::database::connection::{DatabaseConnection, DatabasePool};

/// An error type for Diesel stores.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Diesel error: {0}")]
    Database(#[from] database::connection::Error),

    #[error("Error encoding session data: {0}")]
    Encode(serde_json::Error),

    #[error("Error decoding session data: {0}")]
    Decode(serde_json::Error),

    #[error("Error parsing session ID: {0}")]
    DecodeSlice(<Id as std::str::FromStr>::Err),
}

impl From<Error> for session_store::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Database(inner) => session_store::Error::Backend(inner.to_string()),
            Error::Encode(inner) => session_store::Error::Backend(inner.to_string()),
            Error::Decode(inner) => session_store::Error::Backend(inner.to_string()),
            Error::DecodeSlice(inner) => session_store::Error::Backend(inner.to_string()),
        }
    }
}

pub fn time_to_jiff(expiry_date: time::OffsetDateTime) -> jiff::Timestamp {
    // if we can't convert the expiry date to a timestamp, return the current
    // time i.e. effectively assume our session has expired
    jiff::Timestamp::new(
        expiry_date.unix_timestamp(),
        expiry_date.nanosecond() as i32,
    )
    .unwrap_or(jiff::Timestamp::now())
}

pub fn jiff_to_time(expiry_date: jiff::Timestamp) -> time::OffsetDateTime {
    time::OffsetDateTime::from_unix_timestamp_nanos(expiry_date.as_nanosecond())
        .unwrap_or(time::OffsetDateTime::now_utc())
}

/// A PostgreSQL session store.
#[derive(Clone, Debug)]
pub struct PostgresStore {
    pool: DatabasePool,
    // schema_name: String,
    // table_name: String,
}

impl PostgresStore {
    /// Create a new PostgreSQL store with the provided connection pool.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};
    ///
    /// # tokio_test::block_on(async {
    /// let database_url = std::option_env!("DATABASE_URL").unwrap();
    /// let pool = PgPool::connect(database_url).await.unwrap();
    /// let session_store = PostgresStore::new(pool);
    /// # })
    /// ```
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    async fn id_exists(
        &self,
        conn: &mut DatabaseConnection,
        id: &Id,
    ) -> session_store::Result<bool> {
        session_exists(conn, &id.to_string())
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?
            .pipe(Ok)
    }

    async fn save_with_conn(
        &self,
        conn: &mut DatabaseConnection,
        record: &Record,
    ) -> session_store::Result<()> {
        let json = serde_json::to_value(&record.data).map_err(Error::Encode)?;

        create_or_update_session(
            conn,
            &record.id.to_string(),
            json,
            time_to_jiff(record.expiry_date),
        )
        .await
        .map_err(database::connection::Error::from)
        .map_err(Error::Database)?;

        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        delete_expired(&mut conn)
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        Ok(())
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        while self.id_exists(&mut conn, &record.id).await? {
            record.id = Id::default();
        }
        self.save_with_conn(&mut conn, record).await?;
        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;
        self.save_with_conn(&mut conn, record).await?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;

        load_session(&mut conn, &session_id.to_string())
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?
            .map_or(Ok(None), |session| {
                Record {
                    id: session.id.parse().map_err(Error::DecodeSlice)?,
                    data: serde_json::from_value(session.data).map_err(Error::Decode)?,
                    expiry_date: jiff_to_time(session.expiry_date.to_jiff()),
                }
                .pipe(Some)
                .pipe(Ok)
            })
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?;
        delete_session(&mut conn, &session_id.to_string())
            .await
            .map_err(database::connection::Error::from)
            .map_err(Error::Database)?
            .pipe(Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::tz::TimeZone;
    use time::{Month, macros::datetime};

    #[test]
    fn test_session_id_parsing() {
        let id = Id::default();
        assert_eq!(
            id.to_string()
                .parse::<Id>()
                .map_err(Error::DecodeSlice)
                .unwrap(),
            id
        );
        assert!(
            "invalid!"
                .parse::<Id>()
                .map_err(Error::DecodeSlice)
                .is_err()
        );
    }

    #[test]
    fn test_time_to_jiff() {
        let expiry_date = datetime!(2021-01-01 00:00:00).assume_utc();
        let jiff_date = time_to_jiff(expiry_date);
        let zoned = jiff_date.to_zoned_datetime(TimeZone::UTC);
        assert_eq!(zoned.year(), 2021);
        assert_eq!(zoned.month(), 1);
        assert_eq!(zoned.day(), 1);
        assert_eq!(zoned.hour(), 0);
        assert_eq!(zoned.minute(), 0);
        assert_eq!(zoned.second(), 0);
        assert_eq!(zoned.subsec_nanosecond(), 0);
    }

    #[test]
    fn test_jiff_to_time() {
        let jiff_date: jiff::Timestamp = "2021-01-01T00:00:00Z".parse().unwrap();

        let expiry_date = jiff_to_time(jiff_date);
        assert_eq!(expiry_date.year(), 2021);
        assert_eq!(expiry_date.month(), Month::January);
        assert_eq!(expiry_date.day(), 1);
        assert_eq!(expiry_date.hour(), 0);
        assert_eq!(expiry_date.minute(), 0);
        assert_eq!(expiry_date.second(), 0);
        assert_eq!(expiry_date.nanosecond(), 0);
    }
}
