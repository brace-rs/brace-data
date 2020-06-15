use bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Config, Error, NoTls};

use brace_util_future::result::FutureResult;

pub type FutureConnection<'a> =
    FutureResult<'a, PooledConnection<'a, PostgresConnectionManager<NoTls>>, RunError<Error>>;

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

    pub fn connect(&self) -> FutureConnection {
        FutureResult::from_future(async move { self.pool().get().await })
    }
}
