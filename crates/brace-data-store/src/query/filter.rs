use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use brace_util_future::stream::FutureStream;

pub struct Filter<'a, T, P>(Pin<Box<FutureStream<'a, T>>>, PhantomData<&'a P>);

impl<'a, T, P> Filter<'a, T, P> {
    pub fn from_future<F, S>(future: F) -> Self
    where
        F: Future<Output = S> + 'a,
        S: Stream<Item = T> + 'a,
        T: 'a,
    {
        Self(Box::pin(FutureStream::from_future(future)), PhantomData)
    }

    pub fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = T> + 'a,
    {
        Self(Box::pin(FutureStream::from_stream(stream)), PhantomData)
    }
}

impl<'a, T, P> Stream for Filter<'a, T, P>
where
    T: 'a,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
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
