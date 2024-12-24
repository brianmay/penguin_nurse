use diesel::pg::Pg;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::mobc::Pool;
use diesel_async::pooled_connection::mobc::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::PoolError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use thiserror::Error;

use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DatabasePool = Pool<AsyncPgConnection>;
pub type DatabaseConnection = PooledConnection<AsyncPgConnection>;

/// An error type for SQLx stores.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    Mobc(#[from] mobc::Error<PoolError>),

    #[error(transparent)]
    Encode(serde_json::Error),

    #[error(transparent)]
    Decode(serde_json::Error),

    #[error(transparent)]
    DecodeSlice(#[from] base64::DecodeSliceError),
}

async fn run_migrations<A>(async_connection: A) -> Result<(), Box<dyn std::error::Error>>
where
    A: AsyncConnection<Backend = Pg> + 'static,
{
    let mut async_wrapper: AsyncConnectionWrapper<A> =
        AsyncConnectionWrapper::from(async_connection);

    tokio::task::spawn_blocking(move || {
        async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub async fn init() -> DatabasePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);

    let pool = Pool::new(config);

    let mut tries = 0;

    loop {
        let conn = pool.get().await;
        match conn {
            Ok(conn) => {
                run_migrations(conn).await.unwrap();
                break;
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        tries += 1;
        if tries > 10 {
            panic!("Failed to connect to database after 10 tries");
        }
    }

    pool
}
