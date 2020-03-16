use crate::constraint::{Constrain, Error};
use crate::types::text::Text;

pub struct MinLength(pub usize);

impl Constrain<Text> for MinLength {
    fn constrain(&self, data: &Text) -> Result<(), Error> {
        if data.len() < self.0 {
            return Err(Error::message(format!(
                "Does not meet minimum length of {}",
                self.0
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MinLength;
    use crate::constraint::Validate;
    use crate::types::text::Text;

    #[test]
    fn test_text_min_length() {
        let text = Text::from("hello");

        assert!(text.validate(&MinLength(4)).is_ok());
        assert!(text.validate(&MinLength(5)).is_ok());
        assert!(text.validate(&MinLength(6)).is_err());
    }
}
