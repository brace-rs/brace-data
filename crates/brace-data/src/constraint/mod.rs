pub use self::error::Error;

pub mod error;

pub trait Constrain<T> {
    fn constrain(&self, data: &T) -> Result<(), Error>;
}

pub trait Validate<T> {
    fn validate(&self, constraint: &T) -> Result<(), Error>;
}

impl<T, U> Validate<U> for T
where
    U: Constrain<T>,
{
    fn validate(&self, constraint: &U) -> Result<(), Error> {
        constraint.constrain(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Constrain, Error, Validate};

    struct Data(usize);
    struct ConstraintOne(usize);
    struct ConstraintTwo(usize);

    impl Constrain<Data> for ConstraintOne {
        fn constrain(&self, data: &Data) -> Result<(), Error> {
            if self.0 != data.0 {
                return Err(Error::message("Value does not match"));
            }

            Ok(())
        }
    }

    impl Validate<ConstraintTwo> for Data {
        fn validate(&self, constraint: &ConstraintTwo) -> Result<(), Error> {
            if self.0 != constraint.0 {
                return Err(Error::message("Value does not match"));
            }

            Ok(())
        }
    }

    #[test]
    fn test_constrain() {
        let constraint = ConstraintOne(1);

        assert!(constraint.constrain(&Data(1)).is_ok());
        assert!(constraint.constrain(&Data(2)).is_err());
    }

    #[test]
    fn test_validate() {
        let data = Data(1);

        assert!(data.validate(&ConstraintOne(1)).is_ok());
        assert!(data.validate(&ConstraintOne(2)).is_err());
        assert!(data.validate(&ConstraintTwo(1)).is_ok());
        assert!(data.validate(&ConstraintTwo(2)).is_err());
    }
}
