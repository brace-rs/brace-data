use self::definition::Definition;
use crate::constraint::error::Error;

pub mod definition;
pub mod types;

pub trait Data {
    type Definition: Definition;

    fn validate(&self) -> Result<(), Error>;
}
