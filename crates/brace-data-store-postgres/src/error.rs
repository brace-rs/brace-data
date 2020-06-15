use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    TimedOut,
    User(tokio_postgres::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TimedOut => write!(f, "timed out"),
            Self::User(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Self {
        Self::User(error)
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for Error {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        match error {
            bb8::RunError::TimedOut => Self::TimedOut,
            bb8::RunError::User(err) => Self::User(err),
        }
    }
}
