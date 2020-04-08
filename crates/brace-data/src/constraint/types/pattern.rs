use regex::Regex;

use crate::constraint::{Constrain, Error};
use crate::data::definition::Definition;
use crate::data::types::text::Text;
use crate::data::Data;

#[derive(Clone, Debug)]
pub struct Pattern(Regex);

impl Pattern {
    pub fn new<T>(pattern: T) -> Result<Self, regex::Error>
    where
        T: AsRef<str>,
    {
        Ok(Self(Regex::new(pattern.as_ref())?))
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Constrain<Text> for Pattern {
    fn constrain(&self, data: &Text) -> Result<(), Error> {
        if !self.0.is_match(&data.0) {
            return Err(Error::message(format!(
                "{} does not match pattern {}",
                data.definition().label(),
                self.0.as_str(),
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;
    use crate::constraint::Validate;
    use crate::data::types::text::Text;

    #[test]
    fn test_pattern_equality() {
        let a = Pattern::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
        let b = Pattern::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
        let c = Pattern::new("[0-9]{3}-[0-9]{3}-[0-9]{5}").unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn test_text_pattern() {
        let text = Text::from("hello");
        let number = Text::from("111-222-3333");
        let pattern = Pattern::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();

        assert!(text.validate(&pattern).is_err());
        assert!(number.validate(&pattern).is_ok());
    }
}
