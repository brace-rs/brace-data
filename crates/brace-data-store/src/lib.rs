pub use self::query::select::Select;

pub mod query;

pub trait Store {
    type Item;

    fn select(&self) -> Select<'_, &Self::Item>;
}

#[cfg(test)]
mod tests {
    use futures::stream::{iter, StreamExt};
    use indexmap::IndexSet;

    use super::{Select, Store};

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Book(&'static str);

    struct Books(IndexSet<Book>);

    impl Store for Books {
        type Item = Book;

        fn select(&self) -> Select<'_, &Self::Item> {
            Select::from_stream(iter(self.0.iter()))
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
}
