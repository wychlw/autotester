//! [`DeANSI`] is a wrapper for [`Tty`] that removes ANSI escape sequences from the input and output.

use std::error::Error;

use crate::{impl_any, vendor::strip_ansi_escapes};

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

impl_any!(DeANSI);

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
    fn exit(self) -> DynTty {
        self.inner
    }

    fn inner_ref(&self) -> &DynTty {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut DynTty {
        &mut self.inner
    }
}
