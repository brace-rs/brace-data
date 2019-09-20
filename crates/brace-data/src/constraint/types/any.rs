use std::any::TypeId;
use std::collections::HashMap;

use crate::constraint::{Constrain, Constraint, Error};

#[derive(Default)]
pub struct Any<T>(HashMap<TypeId, Box<dyn Constraint<T>>>);

impl<T> Any<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<U>(&mut self, constraint: U)
    where
        U: Constraint<T> + 'static,
    {
        self.0.insert(TypeId::of::<U>(), Box::new(constraint));
    }

    pub fn remove<U>(&mut self)
    where
        U: Constraint<T> + 'static,
    {
        self.0.remove(&TypeId::of::<U>());
    }
}

impl<T> Constrain<T> for Any<T> {
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in self.0.values() {
            if let Ok(()) = (**constraint).constrain(data) {
                return Ok(());
            }
        }

        Err(Error::message("No constraints passed validation"))
    }
}

#[cfg(test)]
mod tests {
    use super::Any;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;

    #[test]
    fn test_any() {
        let text = String::from("hello");

        let mut constraint_one = Any::new();

        constraint_one.insert(MinLength(9));
        constraint_one.insert(MaxLength(9));

        assert!(text.validate(&constraint_one).is_ok());

        let mut constraint_two = Any::new();

        constraint_two.insert(MinLength(9));
        constraint_two.insert(MaxLength(1));

        assert!(text.validate(&constraint_two).is_err());
    }
}
