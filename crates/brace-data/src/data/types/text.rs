use crate::constraint::{Constraints, Error, ValidateConstraint};
use crate::data::definition::Definition;
use crate::data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Text(String, TextDefinition);

impl Text {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into(), TextDefinition::default())
    }

    pub fn with<T>(value: T, definition: TextDefinition) -> Self
    where
        T: Into<String>,
    {
        Self(value.into(), definition)
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
    use crate::{Data, Definition, TextDefinition};

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
        let text = Text::with("hello", {
            let mut definition = TextDefinition::new();
            definition.constraints_mut().insert(MinLength(1));
            definition.constraints_mut().insert(MaxLength(9));
            definition
        });

        assert!(text.validate().is_ok());

        let text = Text::with("hello", {
            let mut definition = TextDefinition::new();
            definition.constraints_mut().insert(MinLength(9));
            definition.constraints_mut().insert(MaxLength(9));
            definition
        });

        assert!(text.validate().is_err());
    }
}
