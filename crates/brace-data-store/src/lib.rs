pub use self::query::filter::{Filter, FutureFilter, Predicate};
pub use self::query::select::{FutureSelect, Select};
pub use self::record::{Record, Records};

pub mod query;
pub mod record;

pub trait Store {
    type Item;

    fn select<'a>(&'a self) -> FutureSelect<'a, Self::Item>
    where
        Self: Select<'a, <Self as Store>::Item>,
    {
        FutureSelect::new(Select::execute(self))
    }

    fn filter<'a, P>(&'a self, predicate: P) -> FutureFilter<'a, Self::Item, P>
    where
        P: Predicate<Self::Item> + Copy,
        Self: Filter<'a, <Self as Store>::Item, P>,
    {
        FutureFilter::new(Filter::execute(self, predicate))
    }
}

#[cfg(test)]
mod tests {
    use futures::future::{ok, Ready};
    use futures::stream::{iter, StreamExt};
    use indexmap::IndexSet;

    use brace_util_future::result::FutureResult;

    use crate::query::filter::{self, Filter, Predicate};
    use crate::query::select::{self, Select};
    use crate::{Record, Records, Store};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Book(&'static str);

    struct Books(IndexSet<Book>);

    impl Store for Books {
        type Item = Book;
    }

    impl<'a> Select<'a, Book> for Books {
        type Output = FutureResult<'a, Records<'a, Book>, select::Error>;

        fn execute(&'a self) -> Self::Output {
            FutureResult::from_future(async move {
                Ok(Records::from_stream(iter(
                    self.0.iter().map(|item| Record::new(item.clone())),
                )))
            })
        }
    }

    impl<'a, P> Filter<'a, Book, P> for Books
    where
        P: Predicate<Book> + Copy + 'a,
    {
        type Output = Ready<Result<Records<'a, Book>, filter::Error>>;

        fn execute(&'a self, predicate: P) -> Self::Output {
            ok(Records::from_stream(
                iter(self.0.iter().map(|item| Record::new(item.clone()))).filter_map(
                    move |item| async move {
                        if predicate.test(&item) {
                            return Some(item);
                        }

                        None
                    },
                ),
            ))
        }
    }

    impl Default for Books {
        fn default() -> Self {
            let mut set = IndexSet::new();
            set.insert(Book("1984"));
            set.insert(Book("Frankenstein"));
            set.insert(Book("To Kill a Mockingbird"));
            Self(set)
        }
    }

    #[tokio::test]
    async fn test_books_select() -> Result<(), select::Error> {
        let store = Books::default();
        let mut books = store.select().await?;

        assert_eq!(books.next().await, Some(Record::new(Book("1984"))));
        assert_eq!(books.next().await, Some(Record::new(Book("Frankenstein"))));
        assert_eq!(
            books.next().await,
            Some(Record::new(Book("To Kill a Mockingbird")))
        );
        assert_eq!(books.next().await, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_books_filter() -> Result<(), filter::Error> {
        let store = Books::default();
        let mut books = store.filter(|item: &Book| item.0 == "Frankenstein").await?;

        assert_eq!(books.next().await, Some(Record::new(Book("Frankenstein"))));
        assert_eq!(books.next().await, None);

        Ok(())
    }
}
