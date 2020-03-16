use crate::constraint::{Constrain, Constraint, Constraints, Error};
use crate::Data;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Or<T>(Constraints<T>)
where
    T: Data;

impl<T> Or<T>
where
    T: Data,
{
    pub fn new() -> Self {
        Self(Constraints::new())
    }

    pub fn insert<U>(&mut self, constraint: U)
    where
        U: Constraint<T> + 'static,
    {
        self.0.insert(constraint);
    }

    pub fn remove<U>(&mut self)
    where
        U: Constraint<T> + 'static,
    {
        self.0.remove::<U>()
    }
}

impl<T> Constrain<T> for Or<T>
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in &self.0 {
            if let Ok(()) = (**constraint).constrain(data) {
                return Ok(());
            }
        }

        Err(Error::message("No constraints passed validation"))
    }
}

#[cfg(test)]
mod tests {
    use super::Or;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;
    use crate::data::types::text::Text;

    #[test]
    fn test_or() {
        let text = Text::from("hello");

        let mut constraint_one = Or::new();

        constraint_one.insert(MinLength(9));
        constraint_one.insert(MaxLength(9));

        assert!(text.validate(&constraint_one).is_ok());

        let mut constraint_two = Or::new();

        constraint_two.insert(MinLength(9));
        constraint_two.insert(MaxLength(1));

        assert!(text.validate(&constraint_two).is_err());
    }
}
