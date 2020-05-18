pub use self::query::filter::{Filter, Predicate};
pub use self::query::select::Select;
pub use self::record::{Record, Records};

pub mod query;
pub mod record;

pub trait Store {
    type Item;

    fn select(&self) -> Select<'_, Self::Item>;

    fn filter<P>(&self, predicate: P) -> Filter<'_, Self::Item, P>
    where
        P: Predicate<Self::Item> + Copy;
}

#[cfg(test)]
mod tests {
    use futures::stream::{iter, StreamExt};
    use indexmap::IndexSet;

    use crate::query::filter::{self, Filter, Predicate};
    use crate::query::select::{self, Select};
    use crate::{Record, Records, Store};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Book(&'static str);

    struct Books(IndexSet<Book>);

    impl Store for Books {
        type Item = Book;

        fn select(&self) -> Select<'_, Self::Item> {
            Select::from_future(async move {
                Ok(Records::from_stream(iter(
                    self.0.iter().map(|item| Record::new(item.clone())),
                )))
            })
        }

        fn filter<P>(&self, predicate: P) -> Filter<'_, Self::Item, P>
        where
            P: Predicate<Self::Item> + Copy,
        {
            Filter::from_result(Ok(Records::from_stream(
                iter(self.0.iter().map(|item| Record::new(item.clone()))).filter_map(
                    move |item| async move {
                        if predicate.test(&item) {
                            return Some(item);
                        }

                        None
                    },
                ),
            )))
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
