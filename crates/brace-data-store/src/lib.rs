pub use self::query::filter::{Filter, Predicate};
pub use self::query::select::Select;

pub mod query;

pub trait Store {
    type Item;

    fn select(&self) -> Select<'_, &Self::Item>;

    fn filter<P>(&self, predicate: P) -> Filter<'_, &Self::Item, P>
    where
        P: Predicate<Self::Item> + Copy;
}

#[cfg(test)]
mod tests {
    use futures::stream::{iter, StreamExt};
    use indexmap::IndexSet;

    use super::{Filter, Predicate, Select, Store};

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Book(&'static str);

    struct Books(IndexSet<Book>);

    impl Store for Books {
        type Item = Book;

        fn select(&self) -> Select<'_, &Self::Item> {
            Select::from_stream(iter(self.0.iter()))
        }

        fn filter<P>(&self, predicate: P) -> Filter<'_, &Self::Item, P>
        where
            P: Predicate<Self::Item> + Copy,
        {
            Filter::from_stream(iter(self.0.iter()).filter_map(move |item| async move {
                if predicate.test(item) {
                    return Some(item);
                }

                None
            }))
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
    async fn test_books_select() {
        let store = Books::default();
        let mut books = store.select();

        assert_eq!(books.next().await, Some(&Book("1984")));
        assert_eq!(books.next().await, Some(&Book("Frankenstein")));
        assert_eq!(books.next().await, Some(&Book("To Kill a Mockingbird")));
        assert_eq!(books.next().await, None);
    }

    #[tokio::test]
    async fn test_books_filter() {
        let store = Books::default();
        let mut books = store.filter(|item: &Book| item.0 == "Frankenstein");

        assert_eq!(books.next().await, Some(&Book("Frankenstein")));
        assert_eq!(books.next().await, None);
    }
}
