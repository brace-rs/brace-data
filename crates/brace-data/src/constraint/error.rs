use std::fmt;

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl Error {
    pub fn message<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self::Message(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}
