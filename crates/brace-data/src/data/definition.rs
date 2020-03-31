use super::Data;
use crate::constraint::{Constraint, Constraints};

pub trait Definition: Default {
    type Data: Data;

    fn label(&self) -> &str;

    fn set_label<T>(&mut self, label: T)
    where
        T: Into<String>;

    fn with_label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.set_label(label);
        self
    }

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
