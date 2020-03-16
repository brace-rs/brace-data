use self::definition::Definition;

pub mod definition;
pub mod types;

pub trait Data {
    type Definition: Definition;
}
