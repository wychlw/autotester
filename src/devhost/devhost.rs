use std::sync::{Arc, Mutex};

pub trait DevHost<T> {
    fn get_device() -> Arc<Mutex<T>>;
}
