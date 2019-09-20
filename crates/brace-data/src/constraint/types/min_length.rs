use crate::constraint::{Constrain, Error};

pub struct MinLength(pub usize);

impl Constrain<String> for MinLength {
    fn constrain(&self, data: &String) -> Result<(), Error> {
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

    #[test]
    fn test_min_length() {
        let text = String::from("hello");

        assert!(text.validate(&MinLength(4)).is_ok());
        assert!(text.validate(&MinLength(5)).is_ok());
        assert!(text.validate(&MinLength(6)).is_err());
    }
}
