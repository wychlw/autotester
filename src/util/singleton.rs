#[macro_export]
macro_rules! singleton {
    ($name:ident, $t:ty, $init:expr) => {
        pub struct $name {
        }

        impl $name {

            pub fn get() -> &'static mut $t {
                static mut INSTANCE: Option<$t> = None;
                unsafe {
                    INSTANCE.get_or_insert_with(|| $init)
                }
            }
        }
    };
}