use crate::constraint::{Constraints, Error, ValidateConstraint};
use crate::data::definition::Definition;
use crate::data::{Construct, Data};

#[derive(Clone, Debug, PartialEq)]
pub struct Text(String, TextDefinition);

impl Text {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into(), TextDefinition::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Data for Text {
    type Definition = TextDefinition;

    fn definition(&self) -> &Self::Definition {
        &self.1
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_constraint(self.definition())
    }
}

impl Construct for Text {
    type Value = String;

    fn construct<T>(value: T, definition: Self::Definition) -> Self
    where
        T: Into<Self::Value>,
    {
        Self(value.into(), definition)
    }
}

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self::new(from)
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self::new(from)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextDefinition {
    constraints: Constraints<Text>,
}

impl TextDefinition {
    pub fn new() -> Self {
        Self {
            constraints: Constraints::new(),
        }
    }
}

impl Definition for TextDefinition {
    type Data = Text;

    fn constraints(&self) -> &Constraints<Self::Data> {
        &self.constraints
    }

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
        &mut self.constraints
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::{Construct, Data, Definition, TextDefinition};

    #[test]
    fn test_text_length() {
        let text = Text::new("");

        assert_eq!(text.len(), 0);
        assert!(text.is_empty());

        let text = Text::new("hello world");

        assert_eq!(text.len(), 11);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_text_from() {
        let text1 = Text::from("hello");
        let text2 = Text::from(String::from("hello"));

        assert_eq!(text1, text2);
    }

    #[test]
    fn test_text_data_definition() {
        let text = Text::construct("hello", {
            Text::define()
                .with_constraint(MinLength(1))
                .with_constraint(MaxLength(9))
        });

        assert!(text.validate().is_ok());

        let text = Text::construct("hello", {
            Text::define()
                .with_constraint(MinLength(9))
                .with_constraint(MaxLength(9))
        });

        assert!(text.validate().is_err());

        let mut definition = TextDefinition::new();
        let constraints = definition.constraints_mut();

        constraints.insert(MinLength(2));
        constraints.insert(MaxLength(3));

        assert_eq!(
            definition,
            Text::define()
                .with_constraint(MinLength(2))
                .with_constraint(MaxLength(3))
        );
    }
}
