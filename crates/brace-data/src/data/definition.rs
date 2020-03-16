use super::Data;
use crate::constraint::Constraints;

pub trait Definition {
    type Data: Data;

    fn constraints(&self) -> &Constraints<Self::Data>;

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data>;
}

#[derive(Default)]
pub struct SimpleDefinition<T: Data> {
    constraints: Constraints<T>,
}

impl<T> SimpleDefinition<T>
where
    T: Data,
{
    pub fn new() -> Self {
        Self {
            constraints: Constraints::new(),
        }
    }
}

impl<T> Definition for SimpleDefinition<T>
where
    T: Data,
{
    type Data = T;

    fn constraints(&self) -> &Constraints<Self::Data> {
        &self.constraints
    }

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data> {
        &mut self.constraints
    }
}
