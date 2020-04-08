use crate::constraint::Constraints;
use crate::data::definition::Definition;
use crate::data::{Construct, Data};

#[derive(Clone, Debug, PartialEq)]
pub struct Text(pub(crate) String, TextDefinition);

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

#[derive(Clone, Debug, PartialEq)]
pub struct TextDefinition {
    label: String,
    constraints: Constraints<Text>,
}

impl TextDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Definition for TextDefinition {
    type Data = Text;

    fn label(&self) -> &str {
        &self.label
    }

    fn set_label<T>(&mut self, label: T)
    where
        T: Into<String>,
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

impl Default for TextDefinition {
    fn default() -> Self {
        Self {
            label: String::from("Text"),
            constraints: Constraints::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::constraint::types::max_length::MaxLength;
    use crate::constraint::types::min_length::MinLength;
    use crate::constraint::Validate;
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
                .with_label("Greeting")
                .with_constraint(MinLength(1))
                .with_constraint(MaxLength(9))
        });

        assert!(text.validate(text.definition()).is_ok());
        assert_eq!(text.definition().label(), "Greeting");

        let text = Text::construct("hello", {
            Text::define()
                .with_label("Greeting")
                .with_constraint(MinLength(9))
                .with_constraint(MaxLength(9))
        });

        assert!(text.validate(text.definition()).is_err());
        assert_eq!(text.definition().label(), "Greeting");

        let mut definition = TextDefinition::new();
        let constraints = definition.constraints_mut();

        constraints.insert(MinLength(2));
        constraints.insert(MaxLength(3));
        definition.set_label("Message");

        assert_eq!(
            definition,
            Text::define()
                .with_label("Message")
                .with_constraint(MinLength(2))
                .with_constraint(MaxLength(3))
        );
    }
}
