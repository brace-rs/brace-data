use crate::constraint::{Constrain, Error};
use crate::data::definition::Definition;
use crate::data::types::list::List;
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

impl<T> Constrain<List<T>> for MinLength
where
    T: Data,
{
    fn constrain(&self, data: &List<T>) -> Result<(), Error> {
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
    use crate::constraint::Validate;
    use crate::data::types::list::List;
    use crate::data::types::text::Text;

    #[test]
    fn test_text_min_length() {
        let text = Text::from("hello");

        assert!(text.validate(&MinLength(4)).is_ok());
        assert!(text.validate(&MinLength(5)).is_ok());
        assert!(text.validate(&MinLength(6)).is_err());
    }

    #[test]
    fn test_list_min_length() {
        let mut list = List::<Text>::new();

        assert!(list.validate(&MinLength(0)).is_ok());
        assert!(list.validate(&MinLength(1)).is_err());

        list.push(Text::new("hello"));

        assert!(list.validate(&MinLength(0)).is_ok());
        assert!(list.validate(&MinLength(1)).is_ok());
        assert!(list.validate(&MinLength(2)).is_err());
    }
}
