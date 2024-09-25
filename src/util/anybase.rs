use std::{any::Any, rc::Rc};

pub trait AnyBase: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any>;
}

#[macro_export]
macro_rules! impl_any {
    ($name: ident) => {
        impl $crate::util::anybase::AnyBase for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn into_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
        }
    };
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

pub fn heap_raw<T>(t: T) -> *mut T {
    Box::into_raw(Box::new(t))
}
