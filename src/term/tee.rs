use std::{any::Any, fs::File, io::Write};

use crate::util::anybase::AnyBase;

use super::tty::{DynTty, Tty, WrapperTty};


pub struct Tee {
    inner: DynTty,
    file: File,
}

impl Tee {
    pub fn build(inner: DynTty, path: &str) -> Tee {
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
        // self.file.write_all(data)?; // tee should not write to file
        Ok(())
    }
}

impl WrapperTty for Tee {
    fn exit(mut self) -> DynTty {
        self.file.flush().unwrap();
        self.inner
    }
}

