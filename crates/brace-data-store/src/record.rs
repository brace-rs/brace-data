use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use brace_util_future::stream::FutureStream;

#[derive(Clone, Debug, PartialEq)]
pub struct Record<T>(T);

impl<T> Record<T> {
    pub fn new(record: T) -> Self {
        Self(record)
    }
}

impl<T> Deref for Record<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Record<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Records<'a, T>(Pin<Box<FutureStream<'a, Record<T>>>>);

impl<'a, T> Records<'a, T> {
    pub fn from_future<F, S>(future: F) -> Self
    where
        F: Future<Output = S> + 'a,
        S: Stream<Item = Record<T>> + 'a,
        T: 'a,
    {
        Self(Box::pin(FutureStream::from_future(future)))
    }

    pub fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = Record<T>> + 'a,
    {
        Self(Box::pin(FutureStream::from_stream(stream)))
    }
}

impl<'a, T> Stream for Records<'a, T>
where
    T: 'a,
{
    type Item = Record<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
