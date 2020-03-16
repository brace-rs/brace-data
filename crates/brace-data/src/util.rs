use std::any::Any;

pub trait DynPartialEq {
    fn as_any(&self) -> &dyn Any;

    fn eq_any(&self, other: &dyn Any) -> bool;
}

impl<T> DynPartialEq for T
where
    T: PartialEq<T> + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_any(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>().map_or(false, |item| self == item)
    }
}
