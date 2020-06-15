use std::ops::{Deref, DerefMut};

use bb8::{PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Client, Error, NoTls};

#[cfg(feature = "tls")]
use tokio_postgres_rustls::MakeRustlsConnect;

use brace_data_store::connection::Connection;

pub enum PostgresConnection<'a> {
    Plain(PooledConnection<'a, PostgresConnectionManager<NoTls>>),
    #[cfg(feature = "tls")]
    Secure(PooledConnection<'a, PostgresConnectionManager<MakeRustlsConnect>>),
}

impl<'a> PostgresConnection<'a> {
    pub fn plain(conn: PooledConnection<'a, PostgresConnectionManager<NoTls>>) -> Self {
        Self::Plain(conn)
    }

    #[cfg(feature = "tls")]
    pub fn secure(
        conn: PooledConnection<'a, PostgresConnectionManager<MakeRustlsConnect>>,
    ) -> Self {
        Self::Secure(conn)
    }
}

impl Connection for PostgresConnection<'_> {
    type Error = RunError<Error>;
}

impl Deref for PostgresConnection<'_> {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Plain(conn) => conn.deref(),
            #[cfg(feature = "tls")]
            Self::Secure(conn) => conn.deref(),
        }
    }
}

impl DerefMut for PostgresConnection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Plain(conn) => conn.deref_mut(),
            #[cfg(feature = "tls")]
            Self::Secure(conn) => conn.deref_mut(),
        }
    }
}
