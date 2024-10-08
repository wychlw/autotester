//! Tee is a Tty wrapper that writes all output to a file, in addition to passing it to the inner Tty.
//!
//! # Example
//!
//! ```
//! # use tester::cli::shell::Shell;
//! # use tester::cli::tee::Tee;
//! # use tester::cli::tty::Tty;
//! # use tester::cli::tty::WrapperTty;
//! let s = Shell::build(Some("bash"))?;
//! let mut t = Tee::build(Box::new(s), "/tmp/output.log");
//! t.write(b"echo hello\n");
//! t.read();
//! let _ = t.exit();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!

use std::{fs::File, io::Write};

use crate::{impl_any, info};

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

impl_any!(Tee);

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
