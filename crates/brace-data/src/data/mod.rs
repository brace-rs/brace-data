use self::definition::Definition;
use crate::constraint::error::Error;

pub mod definition;
pub mod types;

pub trait Data {
    type Definition: Definition;

    fn definition(&self) -> &Self::Definition;

    fn validate(&self) -> Result<(), Error>;
}
