//! The base trait for all Tty instances

use std::error::Error;

use crate::{impl_any, util::anybase::AnyBase};

/// The base trait for all Tty instances
///
/// A `Tty` can be seen as a device which can:
/// - Be read from
/// - Be written to
pub trait Tty: AnyBase {
    /// Read data from the Tty
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Read a line from the Tty (terminated by a `\n`)
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Write data to the Tty
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}

/// A dynamic Tty instance
pub type DynTty = Box<dyn Tty + Send>;

/// A trait for wrapping a `Tty` and providing additional functionality
pub trait WrapperTty: Tty {

    /// Exit the Tty and return the inner Tty
    fn exit(self) -> DynTty;

    /// Get a reference to the inner Tty
    fn inner_ref(&self) -> &DynTty;

    /// Get a mutable reference to the inner Tty
    fn inner_mut(&mut self) -> &mut DynTty;
}

pub struct DummyTty {}
impl_any!(DummyTty);
impl Tty for DummyTty {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(Vec::new())
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(Vec::new())
    }
    fn write(&mut self, _data: &[u8]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
