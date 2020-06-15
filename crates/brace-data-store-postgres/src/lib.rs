use futures::future::FutureExt;
use tokio::spawn;
use tokio_postgres::{Client, Config, Error, NoTls};

use brace_util_future::result::FutureResult;

pub struct Postgres(Config);

impl Postgres {
    pub fn new(config: Config) -> Self {
        Self(config)
    }

    pub fn config(&self) -> &Config {
        &self.0
    }

    pub fn connect(&self) -> FutureResult<Client, Error> {
        FutureResult::from_future(async move {
            self.config()
                .connect(NoTls)
                .await
                .map(|(client, connection)| {
                    spawn(connection.map(|_| ()));
                    client
                })
        })
    }
}
