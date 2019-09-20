use crate::constraint::{Constrain, Error};

pub struct MaxLength(pub usize);

impl Constrain<String> for MaxLength {
    fn constrain(&self, data: &String) -> Result<(), Error> {
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

    #[test]
    fn test_max_length() {
        let text = String::from("hello");

        assert!(text.validate(&MaxLength(4)).is_err());
        assert!(text.validate(&MaxLength(5)).is_ok());
        assert!(text.validate(&MaxLength(6)).is_ok());
    }
}
