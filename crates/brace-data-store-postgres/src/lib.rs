use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use futures::future::TryFutureExt;
use tokio_postgres::NoTls;

#[cfg(feature = "tls")]
use tokio_postgres_rustls::MakeRustlsConnect;

use brace_data_store::connection::FutureConnection;

pub use tokio_postgres::Config;

#[cfg(feature = "tls")]
pub use rustls::ClientConfig as TlsConfig;

pub use self::connection::PostgresConnection;
pub use self::error::Error;

pub mod connection;
pub mod error;

#[cfg(feature = "snakeoil")]
const SNAKEOIL_CERT: &[u8] = include_bytes!("../fixtures/server.crt");

#[derive(Clone)]
pub struct Postgres(PostgresInner);

impl Postgres {
    pub fn new(config: Config) -> Self {
        Self(PostgresInner::Plain(Pool::builder().build_unchecked(
            PostgresConnectionManager::new(config, NoTls),
        )))
    }

    #[cfg(feature = "tls")]
    pub fn secure(config: Config, tls_config: TlsConfig) -> Self {
        Self(PostgresInner::Secure(Pool::builder().build_unchecked(
            PostgresConnectionManager::new(config, MakeRustlsConnect::new(tls_config)),
        )))
    }

    #[cfg(all(feature = "tls", feature = "snakeoil"))]
    pub fn snakeoil(config: Config) -> Self {
        use std::io::BufReader;

        let mut tls_config = TlsConfig::new();
        let mut reader = BufReader::new(SNAKEOIL_CERT);

        tls_config.root_store.add_pem_file(&mut reader).ok();

        Self::secure(config, tls_config)
    }

    pub fn connect(&self) -> FutureConnection<PostgresConnection> {
        self.0.connect()
    }
}

impl Default for Postgres {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[derive(Clone, Debug)]
enum PostgresInner {
    Plain(Pool<PostgresConnectionManager<NoTls>>),
    #[cfg(feature = "tls")]
    Secure(Pool<PostgresConnectionManager<MakeRustlsConnect>>),
}

impl PostgresInner {
    fn connect(&self) -> FutureConnection<PostgresConnection> {
        match self {
            Self::Plain(pool) => FutureConnection::from_future(
                pool.get().map_ok(PostgresConnection::plain).err_into(),
            ),
            #[cfg(feature = "tls")]
            Self::Secure(pool) => FutureConnection::from_future(
                pool.get().map_ok(PostgresConnection::secure).err_into(),
            ),
        }
    }
}
