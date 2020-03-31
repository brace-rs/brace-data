use crate::constraint::{Constrain, Error};
use crate::data::definition::Definition;
use crate::data::types::text::Text;
use crate::data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct MaxLength(pub usize);

impl Constrain<Text> for MaxLength {
    fn constrain(&self, data: &Text) -> Result<(), Error> {
        if data.len() > self.0 {
            return Err(Error::message(format!(
                "{} exceeds maximum length of {}",
                data.definition().label(),
                self.0
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MaxLength;
    use crate::constraint::ValidateConstraint;
    use crate::data::types::text::Text;

    #[test]
    fn test_text_max_length() {
        let text = Text::from("hello");

        assert!(text.validate_constraint(&MaxLength(4)).is_err());
        assert!(text.validate_constraint(&MaxLength(5)).is_ok());
        assert!(text.validate_constraint(&MaxLength(6)).is_ok());
    }
}
