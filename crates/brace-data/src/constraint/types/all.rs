use std::any::TypeId;
use std::collections::HashMap;

use crate::constraint::{Constrain, Constraint, Error};

#[derive(Default)]
pub struct All<T>(HashMap<TypeId, Box<dyn Constraint<T>>>);

impl<T> All<T> {
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

impl<T> Constrain<T> for All<T> {
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in self.0.values() {
            (**constraint).constrain(data)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::All;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;

    #[test]
    fn test_all() {
        let text = String::from("hello");

        let mut constraint_one = All::new();

        constraint_one.insert(MinLength(1));
        constraint_one.insert(MaxLength(5));

        assert!(text.validate(&constraint_one).is_ok());

        let mut constraint_two = All::new();

        constraint_two.insert(MinLength(1));
        constraint_two.insert(MaxLength(1));

        assert!(text.validate(&constraint_two).is_err());
    }
}
