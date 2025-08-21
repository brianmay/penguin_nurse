use axum::async_trait;
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

    #[error("Error parsing session expiry date: {0}")]
    DecodeSlice(#[from] base64::DecodeSliceError),
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

pub fn time_to_chrono(expiry_date: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    // if we can't convert the expiry date to a chrono type, return the current time i.e. effectively assume our session has expired
    chrono::DateTime::from_timestamp(expiry_date.unix_timestamp(), expiry_date.nanosecond())
        .unwrap_or(chrono::Utc::now())
}

pub fn chono_to_time(expiry_date: chrono::DateTime<chrono::Utc>) -> time::OffsetDateTime {
    let timestamp = expiry_date.timestamp();
    let subsec_nanos = expiry_date.timestamp_subsec_nanos();

    time::OffsetDateTime::from_unix_timestamp(timestamp)
        .map(|x| x + time::Duration::nanoseconds(subsec_nanos as i64))
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
    /// ```rust,no_run
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
            time_to_chrono(record.expiry_date),
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
                    expiry_date: chono_to_time(session.expiry_date),
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
    use chrono::{Datelike, TimeZone, Timelike};
    use time::{Month, macros::datetime};

    #[test]
    fn test_time_to_chrono() {
        let expiry_date = datetime!(2021-01-01 00:00:00).assume_utc();
        let chrono_date = time_to_chrono(expiry_date);
        assert_eq!(chrono_date.year(), 2021);
        assert_eq!(chrono_date.month(), 1);
        assert_eq!(chrono_date.day(), 1);
        assert_eq!(chrono_date.hour(), 0);
        assert_eq!(chrono_date.minute(), 0);
        assert_eq!(chrono_date.second(), 0);
        assert_eq!(chrono_date.nanosecond(), 0);
    }

    #[test]
    fn test_chrono_to_time() {
        let chrono_date = chrono::Utc
            .with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .single()
            .unwrap();

        let expiry_date = chono_to_time(chrono_date);
        assert_eq!(expiry_date.year(), 2021);
        assert_eq!(expiry_date.month(), Month::January);
        assert_eq!(expiry_date.day(), 1);
        assert_eq!(expiry_date.hour(), 0);
        assert_eq!(expiry_date.minute(), 0);
        assert_eq!(expiry_date.second(), 0);
        assert_eq!(expiry_date.nanosecond(), 0);
    }
}
