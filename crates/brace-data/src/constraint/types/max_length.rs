use crate::constraint::{Constrain, Error};
use crate::data::types::text::Text;

#[derive(Clone)]
pub struct MaxLength(pub usize);

impl Constrain<Text> for MaxLength {
    fn constrain(&self, data: &Text) -> Result<(), Error> {
        if data.len() > self.0 {
            return Err(Error::message(format!(
                "Exceeds maximum length of {}",
                self.0
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MaxLength;
    use crate::constraint::Validate;
    use crate::data::types::text::Text;

    #[test]
    fn test_text_max_length() {
        let text = Text::from("hello");

        assert!(text.validate(&MaxLength(4)).is_err());
        assert!(text.validate(&MaxLength(5)).is_ok());
        assert!(text.validate(&MaxLength(6)).is_ok());
    }
}
