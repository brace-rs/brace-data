use std::error;
use std::fmt::{self, Display};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use brace_util_future::result::FutureResult;

use crate::record::Records;

pub trait Select<'a, T> {
    type Output: Future<Output = Result<Records<'a, T>, Error>>;

    fn execute(&'a self) -> Self::Output;
}

pub struct FutureSelect<'a, T>(Pin<Box<FutureResult<'a, Records<'a, T>, Error>>>);

impl<'a, T> FutureSelect<'a, T> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = Result<Records<'a, T>, Error>> + 'a,
    {
        Self(Box::pin(FutureResult::from_future(future)))
    }
}

impl<'a, T> Future for FutureSelect<'a, T>
where
    T: 'a,
{
    type Output = Result<Records<'a, T>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl Error {
    pub fn message<T>(message: T) -> Self
    where
        T: Into<String>,
    {
        Self::Message(message.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(message) => message.fmt(f),
        }
    }
}

impl error::Error for Error {}
