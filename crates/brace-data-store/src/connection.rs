use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use brace_util_future::result::FutureResult;

pub trait Connection {
    type Error: Error;
}

pub struct FutureConnection<'a, C>(Pin<Box<FutureResult<'a, C, C::Error>>>)
where
    C: Connection;

impl<'a, C> FutureConnection<'a, C>
where
    C: Connection,
{
    pub fn from_future<F>(future: F) -> Self
    where
        F: Future<Output = Result<C, C::Error>> + 'a,
    {
        Self(Box::pin(FutureResult::from_future(future)))
    }

    pub fn ready(ok: C) -> Self {
        Self(Box::pin(FutureResult::from_ok(ok)))
    }

    pub fn error(err: C::Error) -> Self {
        Self(Box::pin(FutureResult::from_err(err)))
    }
}

impl<'a, C> Connection for FutureConnection<'a, C>
where
    C: Connection,
{
    type Error = C::Error;
}

impl<'a, C> Future for FutureConnection<'a, C>
where
    C: Connection,
{
    type Output = Result<C, C::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}
