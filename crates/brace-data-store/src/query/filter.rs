use std::error;
use std::fmt::{self, Display};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use brace_util_future::result::FutureResult;

use crate::record::Records;

pub struct Filter<'a, T, P>(
    Pin<Box<FutureResult<'a, Records<'a, T>, Error>>>,
    PhantomData<&'a P>,
);

impl<'a, T, P> Filter<'a, T, P> {
    pub fn from_result(result: Result<Records<'a, T>, Error>) -> Self {
        Self(Box::pin(FutureResult::from_result(result)), PhantomData)
    }

    pub fn from_future<F>(future: F) -> Self
    where
        F: Future<Output = Result<Records<'a, T>, Error>> + 'a,
    {
        Self(Box::pin(FutureResult::from_future(future)), PhantomData)
    }
}

impl<'a, T, P> Future for Filter<'a, T, P>
where
    T: 'a,
{
    type Output = Result<Records<'a, T>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

pub trait Predicate<T> {
    fn test(&self, item: &T) -> bool;
}

impl<T, U> Predicate<T> for U
where
    U: Fn(&T) -> bool,
{
    fn test(&self, item: &T) -> bool {
        (self)(item)
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
