use std::ops::{Deref, DerefMut};

use crate::constraint::Constraints;
use crate::data::definition::Definition;
use crate::data::{Construct, Data, Define};

#[derive(Clone, Debug, PartialEq)]
pub struct List<T>(Vec<T>, ListDefinition<T>)
where
    T: Data + 'static;

impl<T> List<T>
where
    T: Data + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Data for List<T>
where
    T: Data + 'static,
{
    type Definition = ListDefinition<T>;

    fn definition(&self) -> &Self::Definition {
        &self.1
    }
}

impl<T> Define for List<T>
where
    T: Data + 'static,
{
    fn define() -> Self::Definition {
        Self::Definition::default()
    }
}

impl<T> Construct for List<T>
where
    T: Data + 'static,
{
    type Value = Vec<T>;

    fn construct<U>(value: U, definition: Self::Definition) -> Self
    where
        U: Into<Self::Value>,
    {
        Self(value.into(), definition)
    }
}

impl<T> Default for List<T>
where
    T: Data,
{
    fn default() -> Self {
        Self(Vec::new(), ListDefinition::default())
    }
}

impl<T> Deref for List<T>
where
    T: Data,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}

impl<T> DerefMut for List<T>
where
    T: Data,
{
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}

impl<T> From<Vec<T>> for List<T>
where
    T: Data,
{
    fn from(from: Vec<T>) -> Self {
        Self(from, ListDefinition::default())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListDefinition<T>
where
    T: Data + 'static,
{
    label: String,
    constraints: Constraints<List<T>>,
}

impl<T> ListDefinition<T>
where
    T: Data,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Definition for ListDefinition<T>
where
    T: Data,
{
    type Data = List<T>;

    fn label(&self) -> &str {
        &self.label
    }

    fn set_label<U>(&mut self, label: U)
    where
        U: Into<String>,
    {
        self.label = label.into();
    }

    fn constraints(&self) -> &Constraints<Self::Data> {
        &self.constraints
    }

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
        &mut self.constraints
    }
}

impl<T> Default for ListDefinition<T>
where
    T: Data,
{
    fn default() -> Self {
        Self {
            label: String::from("List"),
            constraints: Constraints::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{List, ListDefinition};
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::Validate;
    use crate::{Construct, Data, Define, Definition, Text};

    #[test]
    fn test_list_data() {
        let mut list = List::<Text>::new();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        list.push(Text::new("one"));
        list.push(Text::new("two"));

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());

        assert_eq!(list.get(0), Some(&Text::new("one")));
        assert_eq!(list.get(1), Some(&Text::new("two")));

        *list.get_mut(0).unwrap() = Text::new("first");

        assert_eq!(list.get(0), Some(&Text::new("first")));
    }

    #[test]
    fn test_list_from() {
        let list = List::from(vec![Text::new("hello"), Text::new("world")]);

        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_list_data_definition() {
        let list = List::<Text>::construct(
            Vec::new(),
            List::define()
                .with_label("Items")
                .with_constraint(MaxLength(2)),
        );

        assert!(list.validate(list.definition()).is_ok());
        assert_eq!(list.definition().label(), "Items");

        let mut definition = ListDefinition::<Text>::new();
        let constraints = definition.constraints_mut();

        constraints.insert(MaxLength(2));
        definition.set_label("Items");

        assert_eq!(
            definition,
            List::<Text>::define()
                .with_label("Items")
                .with_constraint(MaxLength(2))
        );
    }
}
