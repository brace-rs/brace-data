use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use brace_util_future::stream::FutureStream;

pub struct Select<'a, T>(Pin<Box<FutureStream<'a, T>>>);

impl<'a, T> Select<'a, T> {
    pub fn from_future<F, S>(future: F) -> Self
    where
        F: Future<Output = S> + 'a,
        S: Stream<Item = T> + 'a,
        T: 'a,
    {
        Self(Box::pin(FutureStream::from_future(future)))
    }

    pub fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = T> + 'a,
    {
        Self(Box::pin(FutureStream::from_stream(stream)))
    }
}

impl<'a, T> Stream for Select<'a, T>
where
    T: 'a,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}
