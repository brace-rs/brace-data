use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use futures::future::TryFutureExt;
use tokio_postgres::{Error, NoTls};

#[cfg(feature = "tls")]
use tokio_postgres_rustls::MakeRustlsConnect;

use brace_data_store::connection::FutureConnection;
use brace_util_future::result::FutureResult;

pub use tokio_postgres::Config;

#[cfg(feature = "tls")]
pub use rustls::ClientConfig as TlsConfig;

pub use self::connection::PostgresConnection;

pub mod connection;

#[cfg(feature = "snakeoil")]
const SNAKEOIL_CERT: &[u8] = include_bytes!("../fixtures/server.crt");

pub enum Postgres {
    Plain(Pool<PostgresConnectionManager<NoTls>>),
    #[cfg(feature = "tls")]
    Secure(Pool<PostgresConnectionManager<MakeRustlsConnect>>),
}

impl Postgres {
    pub fn new<'a>(config: Config) -> FutureResult<'a, Self, Error> {
        FutureResult::from_future(async {
            let manager = PostgresConnectionManager::new(config, NoTls);
            let pool = Pool::builder().build(manager).await?;

            Ok(Self::Plain(pool))
        })
    }

    #[cfg(feature = "tls")]
    pub fn secure<'a>(config: Config, tls_config: TlsConfig) -> FutureResult<'a, Self, Error> {
        FutureResult::from_future(async {
            let tls = MakeRustlsConnect::new(tls_config);
            let manager = PostgresConnectionManager::new(config, tls);
            let pool = Pool::builder().build(manager).await?;

            Ok(Self::Secure(pool))
        })
    }

    #[cfg(all(feature = "tls", feature = "snakeoil"))]
    pub fn snakeoil<'a>(config: Config) -> FutureResult<'a, Self, Error> {
        use std::io::BufReader;

        let mut tls_config = TlsConfig::new();
        let mut reader = BufReader::new(SNAKEOIL_CERT);

        tls_config.root_store.add_pem_file(&mut reader).unwrap();

        Self::secure(config, tls_config)
    }

    pub fn connect(&self) -> FutureConnection<PostgresConnection> {
        match self {
            Self::Plain(pool) => {
                FutureConnection::from_future(pool.get().map_ok(PostgresConnection::plain))
            }
            #[cfg(feature = "tls")]
            Self::Secure(pool) => {
                FutureConnection::from_future(pool.get().map_ok(PostgresConnection::secure))
            }
        }
    }
}
