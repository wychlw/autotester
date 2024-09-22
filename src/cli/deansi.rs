//! [`DeANSI`] is a wrapper for [`Tty`] that removes ANSI escape sequences from the input and output.

use std::{any::Any, error::Error};

use crate::{util::anybase::AnyBase, vendor::strip_ansi_escapes};

use super::tty::{DynTty, Tty, WrapperTty};

/// A wrapper for [`Tty`] that removes ANSI escape sequences from the input and output.
pub struct DeANSI {
    inner: DynTty,
}

impl DeANSI {
    /// Build a new [`DeANSI`] instance.
    pub fn build(inner: DynTty) -> DeANSI {
        DeANSI { inner }
    }
}

impl AnyBase for DeANSI {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Tty for DeANSI {
    /// Read data from the Tty
    ///
    /// Due to the escape sequences may be cut off in the middle of the buffer,
    /// read the buffer in line is needed.
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Due to the escape sequences may be cut off in the middle of the buffer,
        // read the buffer in line is needed.
        let data = self.inner.read_line()?;
        let data = strip_ansi_escapes::strip(&data);
        Ok(data)
    }

    /// Read a line from the Tty (terminated by a `\n`)
    ///
    /// In DeANSI, the `read_line` and `read` are the same.
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line()?;
        let data = strip_ansi_escapes::strip(&data);
        Ok(data)
    }

    /// Write data to the Tty
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let data = strip_ansi_escapes::strip(data);
        self.inner.write(&data)?;
        Ok(())
    }
}

impl WrapperTty for DeANSI {
    /// Exit the Tty and return the inner Tty
    fn exit(self) -> DynTty {
        self.inner
    }
}
