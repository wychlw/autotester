//! Tee is a Tty wrapper that writes all output to a file, in addition to passing it to the inner Tty.
//!
//! # Example
//!
//! ```
//! let s = Shell::build("bash");
//! let t = Tee::build(s, "output.log");
//! t.write(b"echo hello\n");
//! t.read();
//! let s = t.exit();
//! s.exit();
//! ```
//!

use std::{any::Any, fs::File, io::Write};

use crate::{info, util::anybase::AnyBase};

use super::tty::{DynTty, Tty, WrapperTty};

pub struct Tee {
    inner: DynTty,
    file: File,
}

impl Tee {
    /// Build a new `Tee` instance.
    ///
    /// # Arguments
    ///
    /// - `inner`: The inner Tty instance.
    /// - `path`: The path to the file to write to.
    pub fn build(inner: DynTty, path: &str) -> Tee {
        info!("Teeing to file {}...", path);
        Tee {
            inner,
            file: File::create(path).unwrap(),
        }
    }
}

impl AnyBase for Tee {
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

impl Tty for Tee {
    fn read(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let res: Vec<u8> = self.inner.read()?;
        self.file.write_all(&res)?;
        Ok(res)
    }
    fn read_line(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let res: Vec<u8> = self.inner.read_line()?;
        self.file.write_all(&res)?;
        Ok(res)
    }
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.write(data)?;
        // self.file.write_all(data)?; // tee should not write to file, but for log purpose...
        Ok(())
    }
}

impl WrapperTty for Tee {
    fn exit(mut self) -> DynTty {
        self.file.flush().unwrap();
        self.inner
    }
}
