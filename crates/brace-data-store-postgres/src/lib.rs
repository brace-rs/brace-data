use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use futures::future::TryFutureExt;
use tokio_postgres::{Config, Error, NoTls};

use brace_data_store::connection::FutureConnection;
use brace_util_future::result::FutureResult;

pub use self::connection::PostgresConnection;

pub mod connection;

pub struct Postgres(Pool<PostgresConnectionManager<NoTls>>);

impl Postgres {
    pub fn new<'a>(config: Config) -> FutureResult<'a, Self, Error> {
        FutureResult::from_future(async {
            let manager = PostgresConnectionManager::new(config, NoTls);
            let pool = Pool::builder().build(manager).await?;

            Ok(Self(pool))
        })
    }

    pub fn pool(&self) -> &Pool<PostgresConnectionManager<NoTls>> {
        &self.0
    }

    pub fn connect(&self) -> FutureConnection<PostgresConnection> {
        FutureConnection::from_future(self.pool().get().map_ok(PostgresConnection::new))
    }
}
