use crate::constraint::Constraints;
use crate::data::definition::Definition;
use crate::data::Data;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text(String);

impl Text {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
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
}

impl From<&str> for Text {
    fn from(from: &str) -> Self {
        Self(from.to_owned())
    }
}

impl From<String> for Text {
    fn from(from: String) -> Self {
        Self(from)
    }
}

#[derive(Default)]
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
    use crate::constraint::ValidateConstraint;
    use crate::{Definition, TextDefinition};

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
        let text = Text::new("hello");

        let mut definition_one = TextDefinition::new();

        definition_one.constraints_mut().insert(MinLength(1));
        definition_one.constraints_mut().insert(MaxLength(9));

        assert!(text.validate_constraint(&definition_one).is_ok());

        let mut definition_two = TextDefinition::new();

        definition_two.constraints_mut().insert(MinLength(9));
        definition_two.constraints_mut().insert(MaxLength(9));

        assert!(text.validate_constraint(&definition_two).is_err());
    }
}
