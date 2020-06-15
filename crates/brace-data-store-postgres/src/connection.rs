use std::ops::{Deref, DerefMut};

use bb8::{ManageConnection, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Error, NoTls};

use brace_data_store::connection::Connection;

pub struct PostgresConnection<'a>(PooledConnection<'a, PostgresConnectionManager<NoTls>>);

impl<'a> PostgresConnection<'a> {
    pub fn new(conn: PooledConnection<'a, PostgresConnectionManager<NoTls>>) -> Self {
        Self(conn)
    }
}

impl Connection for PostgresConnection<'_> {
    type Error = RunError<Error>;
}

impl Deref for PostgresConnection<'_> {
    type Target = <PostgresConnectionManager<NoTls> as ManageConnection>::Connection;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for PostgresConnection<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
