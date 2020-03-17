use self::definition::Definition;
use crate::constraint::error::Error;

pub mod definition;
pub mod types;

pub trait Data {
    type Definition: Definition;

    fn definition(&self) -> &Self::Definition;

    fn validate(&self) -> Result<(), Error>;
}

pub trait Construct: Data {
    type Value;

    fn construct<T>(value: T, definition: Self::Definition) -> Self
    where
        T: Into<Self::Value>;
}
