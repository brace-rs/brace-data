use crate::constraint::{Constrain, Error};
use crate::data::definition::Definition;
use crate::data::types::text::Text;
use crate::data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct MinLength(pub usize);

impl Constrain<Text> for MinLength {
    fn constrain(&self, data: &Text) -> Result<(), Error> {
        if data.len() < self.0 {
            return Err(Error::message(format!(
                "{} does not meet minimum length of {}",
                data.definition().label(),
                self.0
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MinLength;
    use crate::constraint::ValidateConstraint;
    use crate::data::types::text::Text;

    #[test]
    fn test_text_min_length() {
        let text = Text::from("hello");

        assert!(text.validate_constraint(&MinLength(4)).is_ok());
        assert!(text.validate_constraint(&MinLength(5)).is_ok());
        assert!(text.validate_constraint(&MinLength(6)).is_err());
    }
}
