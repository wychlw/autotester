use std::{any::Any, error::Error};

use crate::{util::anybase::AnyBase, vendor::strip_ansi_escapes};

use super::tty::{DynTty, Tty, WrapperTty};

pub struct DeANSI {
    inner: DynTty,
}

impl DeANSI {
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
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Due to the escape sequences may be cut off in the middle of the buffer,
        // read the buffer in line is needed.
        let data = self.inner.read_line()?;
        let data = strip_ansi_escapes::strip(&data);
        Ok(data)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = self.inner.read_line()?;
        let data = strip_ansi_escapes::strip(&data);
        Ok(data)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let data = strip_ansi_escapes::strip(&data);
        self.inner.write(&data)?;
        Ok(())
    }
}

impl WrapperTty for DeANSI {
    fn exit(self) -> DynTty {
        self.inner
    }
}
