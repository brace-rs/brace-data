use crate::constraint::{Constrain, Constraint, Constraints, Error};
use crate::Data;

#[derive(Default)]
pub struct And<T: Data>(Constraints<T>);

impl<T> And<T>
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

impl<T> Constrain<T> for And<T>
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in &self.0 {
            (**constraint).constrain(data)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::And;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;
    use crate::data::types::text::Text;

    #[test]
    fn test_and() {
        let text = Text::from("hello");

        let mut constraint_one = And::new();

        constraint_one.insert(MinLength(1));
        constraint_one.insert(MaxLength(5));

        assert!(text.validate(&constraint_one).is_ok());

        let mut constraint_two = And::new();

        constraint_two.insert(MinLength(1));
        constraint_two.insert(MaxLength(1));

        assert!(text.validate(&constraint_two).is_err());
    }
}
