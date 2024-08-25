use std::any::Any;

pub trait AnyBase {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

#[macro_export]
macro_rules! dyn_cast {
    ($x:expr, $t:ty) => {
        $x.as_any().downcast_ref::<$t>()
    };
}

#[macro_export]
macro_rules! dyn_cast_mut {
    ($x:expr, $t:ty) => {
        $x.as_any_mut().downcast_mut::<$t>()
    };
}

#[macro_export]
macro_rules! dyn_into {
    ($x:expr, $t:ty) => {
        $x.into_any().downcast::<$t>()
    };
}
