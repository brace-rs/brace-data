use self::constraint::Constraints;

pub mod constraint;

pub trait Data {
    type Definition: Definition;
}

impl Data for String {
    type Definition = SimpleDefinition<Self>;
}

pub trait Definition {
    type Data: Data;

    fn constraints(&self) -> &Constraints<Self::Data>;
    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data>;
}

#[derive(Default)]
pub struct SimpleDefinition<T: Data> {
    constraints: Constraints<T>,
}

impl<T> SimpleDefinition<T>
where
    T: Data,
{
    pub fn new() -> Self {
        Self {
            constraints: Constraints::new(),
        }
    }
}

impl<T> Definition for SimpleDefinition<T>
where
    T: Data,
{
    type Data = T;

    fn constraints(&self) -> &Constraints<Self::Data> {
        &self.constraints
    }

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
        &mut self.constraints
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;
    use crate::{Definition, SimpleDefinition};

    #[test]
    fn test_string_data_definition() {
        let text = String::from("hello");

        let mut definition_one = SimpleDefinition::new();

        definition_one.constraints_mut().insert(MinLength(1));
        definition_one.constraints_mut().insert(MaxLength(9));

        assert!(text.validate(&definition_one).is_ok());

        let mut definition_two = SimpleDefinition::new();

        definition_two.constraints_mut().insert(MinLength(9));
        definition_two.constraints_mut().insert(MaxLength(9));

        assert!(text.validate(&definition_two).is_err());
    }
}
