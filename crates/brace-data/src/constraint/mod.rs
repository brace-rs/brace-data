use std::any::TypeId;
use std::collections::hash_map::{HashMap, Values, ValuesMut};
use std::fmt::{self, Debug};
use std::vec::IntoIter;

use dyn_clone::{clone_trait_object, DynClone};

use crate::util::DynPartialEq;
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
    U: Data + 'static,
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

pub trait Constraint<T>: Debug + DynClone + DynPartialEq
where
    T: Data,
{
    fn constrain(&self, data: &T) -> Result<(), Error>;
}

clone_trait_object!(<T> Constraint<T>);

impl<T, U> Constraint<U> for T
where
    U: Data + Validate<T>,
    T: Clone + Debug + PartialEq + 'static,
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

impl<T> Debug for Constraints<T>
where
    T: Data,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.0.values()).finish()
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

impl<T> PartialEq for Constraints<T>
where
    T: Data,
{
    fn eq(&self, item: &Self) -> bool {
        if self.0.len() != item.0.len() {
            return false;
        }

        self.0.iter().all(|(key, val)| {
            item.0
                .get(key)
                .map_or(false, |v| val.eq_any(v.as_ref().as_any()))
        })
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

    struct Number(usize, NumberDefinition);

    impl Number {
        pub fn new(value: usize) -> Self {
            Self(value, NumberDefinition::default())
        }
    }

    impl Data for Number {
        type Definition = NumberDefinition;

        fn definition(&self) -> &Self::Definition {
            &self.1
        }
    }

    #[derive(Default)]
    struct NumberDefinition {
        label: String,
        constraints: Constraints<Number>,
    }

    impl Definition for NumberDefinition {
        type Data = Number;

        fn label(&self) -> &str {
            &self.label
        }

        fn set_label<T>(&mut self, label: T)
        where
            T: Into<String>,
        {
            self.label = label.into();
        }

        fn constraints(&self) -> &Constraints<Self::Data> {
            &self.constraints
        }

        fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
            &mut self.constraints
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct ConstraintOne(usize);

    #[derive(Clone, Debug, PartialEq)]
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

        assert!(constraint.constrain(&Number::new(1)).is_ok());
        assert!(constraint.constrain(&Number::new(2)).is_err());
    }

    #[test]
    fn test_validate() {
        let data = Number::new(1);

        assert!(data.validate(&ConstraintOne(1)).is_ok());
        assert!(data.validate(&ConstraintOne(2)).is_err());
        assert!(data.validate(&ConstraintTwo(1)).is_ok());
        assert!(data.validate(&ConstraintTwo(2)).is_err());
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_constraint_clone() {
        let data = Number::new(1);
        let mut a = Constraints::<Number>::new();

        assert!(data.validate(&a).is_ok());

        a.insert(ConstraintOne(1));

        assert!(data.validate(&a).is_ok());

        a.insert(ConstraintTwo(2));

        assert!(data.validate(&a).is_err());

        let b = a.clone();

        assert!(data.validate(&b).is_err());
    }

    #[test]
    fn test_constraint_debug() {
        let mut constraints = Constraints::<Number>::new();

        constraints.insert(ConstraintOne(1));
        constraints.insert(ConstraintTwo(2));

        let debug = format!("{:?}", constraints);

        assert!(debug.contains("ConstraintOne(1)"));
        assert!(debug.contains("ConstraintTwo(2)"));
    }

    #[test]
    fn test_constraint_equality() {
        let a = ConstraintOne(1);
        let b = ConstraintOne(1);
        let c = ConstraintTwo(2);
        let d = ConstraintTwo(3);

        assert_eq!(a, b);
        assert_ne!(c, d);

        let mut one = Constraints::<Number>::new();

        one.insert(a);

        let mut two = Constraints::<Number>::new();

        assert_ne!(one, two);

        two.insert(b);

        assert_eq!(one, two);
        assert_eq!(one, one.clone());

        one.insert(c);
        two.insert(d);

        assert_ne!(one, two);
    }

    #[test]
    fn test_constraints() {
        let mut constraints = Constraints::<Number>::new();

        constraints.insert(ConstraintOne(1));
        constraints.insert(ConstraintTwo(2));

        assert_eq!((&constraints).into_iter().len(), 2);

        constraints.remove::<ConstraintOne>();

        assert_eq!((&constraints).into_iter().len(), 1);

        constraints.remove::<ConstraintTwo>();

        assert_eq!((&constraints).into_iter().len(), 0);
    }
}
