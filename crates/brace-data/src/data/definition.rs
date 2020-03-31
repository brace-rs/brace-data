use super::Data;
use crate::constraint::{Constraint, Constraints};

pub trait Definition: Default {
    type Data: Data;

    fn constraints(&self) -> &Constraints<Self::Data>;

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data>;

    fn with_constraint<T>(mut self, constraint: T) -> Self
    where
        T: Constraint<Self::Data> + 'static,
    {
        self.constraints_mut().insert(constraint);
        self
    }
}
