use std::any::TypeId;
use std::collections::hash_map::{HashMap, Values, ValuesMut};
use std::vec::IntoIter;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{Data, Definition};

pub use self::error::Error;

pub mod error;
pub mod types;

pub trait Constrain<T>
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error>;
}

impl<T, U> Constrain<U> for T
where
    T: Definition<Data = U>,
    U: Data,
{
    fn constrain(&self, data: &U) -> Result<(), Error> {
        Constraint::constrain(self.constraints(), data)
    }
}

pub trait Validate<T> {
    fn validate(&self, constraint: &T) -> Result<(), Error>;
}

impl<T, U> Validate<U> for T
where
    T: Data,
    U: Constrain<T>,
{
    fn validate(&self, constraint: &U) -> Result<(), Error> {
        constraint.constrain(self)
    }
}

pub trait Constraint<T>: DynClone
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error>;
}

clone_trait_object!(<T> Constraint<T>);

impl<T, U> Constraint<U> for T
where
    U: Data + Validate<T>,
    T: Clone,
{
    fn constrain(&self, data: &U) -> Result<(), Error> {
        data.validate(self)
    }
}

pub struct Constraints<T>(HashMap<TypeId, Box<dyn Constraint<T>>>)
where
    T: Data;

impl<T> Constraints<T>
where
    T: Data,
{
    pub fn new() -> Self {
        Self::default()
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

impl<T> Clone for Constraints<T>
where
    T: Data,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Constraints<T>
where
    T: Data,
{
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<T> Constrain<T> for Constraints<T>
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error> {
        for constraint in self.0.values() {
            (**constraint).constrain(data)?;
        }

        Ok(())
    }
}

impl<T> IntoIterator for Constraints<T>
where
    T: Data,
{
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

impl<'a, T> IntoIterator for &'a Constraints<T>
where
    T: Data,
{
    type Item = &'a Box<dyn Constraint<T>>;
    type IntoIter = Values<'a, TypeId, Box<dyn Constraint<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl<'a, T> IntoIterator for &'a mut Constraints<T>
where
    T: Data,
{
    type Item = &'a mut Box<dyn Constraint<T>>;
    type IntoIter = ValuesMut<'a, TypeId, Box<dyn Constraint<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.values_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::{Constrain, Constraints, Error, Validate};
    use crate::{Data, Definition};

    struct Number(usize);

    impl Data for Number {
        type Definition = NumberDefinition;
    }

    struct NumberDefinition {
        constraints: Constraints<Number>,
    }

    impl Definition for NumberDefinition {
        type Data = Number;

        fn constraints(&self) -> &Constraints<Self::Data> {
            &self.constraints
        }

        fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
            &mut self.constraints
        }
    }

    #[derive(Clone)]
    struct ConstraintOne(usize);

    #[derive(Clone)]
    struct ConstraintTwo(usize);

    impl Constrain<Number> for ConstraintOne {
        fn constrain(&self, data: &Number) -> Result<(), Error> {
            if self.0 != data.0 {
                return Err(Error::message("Value does not match"));
            }

            Ok(())
        }
    }

    impl Validate<ConstraintTwo> for Number {
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

        assert!(constraint.constrain(&Number(1)).is_ok());
        assert!(constraint.constrain(&Number(2)).is_err());
    }

    #[test]
    fn test_validate() {
        let data = Number(1);

        assert!(data.validate(&ConstraintOne(1)).is_ok());
        assert!(data.validate(&ConstraintOne(2)).is_err());
        assert!(data.validate(&ConstraintTwo(1)).is_ok());
        assert!(data.validate(&ConstraintTwo(2)).is_err());
    }

    #[test]
    fn test_constraint_clone() {
        let data = Number(1);
        let mut a = Constraints::<Number>::new();

        assert!(data.validate(&a).is_ok());

        a.insert(ConstraintOne(1));

        assert!(data.validate(&a).is_ok());

        a.insert(ConstraintTwo(2));

        assert!(data.validate(&a).is_err());

        let b = a.clone();

        assert!(data.validate(&b).is_err());
    }
}
