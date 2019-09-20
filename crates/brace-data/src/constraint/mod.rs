use std::any::TypeId;
use std::collections::hash_map::{HashMap, Values, ValuesMut};
use std::vec::IntoIter;

pub use self::error::Error;

pub mod error;
pub mod types;

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

pub trait Constraint<T> {
    fn constrain(&self, data: &T) -> Result<(), Error>;
}

impl<T, U> Constraint<U> for T
where
    U: Validate<T>,
{
    fn constrain(&self, data: &U) -> Result<(), Error> {
        data.validate(self)
    }
}

#[derive(Default)]
pub struct Constraints<T>(HashMap<TypeId, Box<dyn Constraint<T>>>);

impl<T> Constraints<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<U>(&mut self, constraint: U)
    where
        U: Constraint<T> + 'static,
    {
        self.0.insert(TypeId::of::<U>(), Box::new(constraint));
    }

    pub fn remove<U>(&mut self)
    where
        U: Constraint<T> + 'static,
    {
        self.0.remove(&TypeId::of::<U>());
    }
}

impl<T> Constrain<T> for Constraints<T> {
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in self.0.values() {
            (**constraint).constrain(data)?;
        }

        Ok(())
    }
}

impl<T> IntoIterator for Constraints<T> {
    type Item = Box<dyn Constraint<T>>;
    type IntoIter = IntoIter<Box<dyn Constraint<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Constraints<T> {
    type Item = &'a Box<dyn Constraint<T>>;
    type IntoIter = Values<'a, TypeId, Box<dyn Constraint<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl<'a, T> IntoIterator for &'a mut Constraints<T> {
    type Item = &'a mut Box<dyn Constraint<T>>;
    type IntoIter = ValuesMut<'a, TypeId, Box<dyn Constraint<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values_mut()
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
