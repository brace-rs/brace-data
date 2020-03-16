use super::Data;
use crate::constraint::Constraints;

pub trait Definition {
    type Data: Data;

    fn constraints(&self) -> &Constraints<Self::Data>;

    fn constraints_mut(&mut self) -> &mut Constraints<Self::Data>;
}
