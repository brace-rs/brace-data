use self::definition::Definition;

pub mod definition;
pub mod types;

pub trait Data {
    type Definition: Definition;

    fn definition(&self) -> &Self::Definition;
}

pub trait Define: Data {
    fn define() -> Self::Definition;
}

pub trait Construct: Data {
    type Value;

    fn construct<T>(value: T, definition: Self::Definition) -> Self
    where
        T: Into<Self::Value>;
}
